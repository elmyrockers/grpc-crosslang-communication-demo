package main


import (
	// "github.com/davecgh/go-spew/spew"
	_ "github.com/joho/godotenv/autoload"

	"context"
	"log"
	"os"
	"fmt"
	
	"net"
	"google.golang.org/grpc/status"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/grpc-servers/go-service/pb/user"
	"database/sql"
	_ "github.com/go-sql-driver/mysql"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/grpc-servers/go-service/db"
)

type userServer struct {
	user.UnimplementedUserServiceServer

	query *db.Queries
}

func safeString(ns sql.NullString) string {
	if ns.Valid { return ns.String }
	return ""
}

func (s *userServer) All(ctx context.Context, req *user.GetRequest) (*user.GetResponse, error) {
	dbUsers, err := s.query.All( context.Background() )
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to query users: %v", err)
	}

	// Re-map dbUsers to pbUsers
		pbUsers := make([]*user.User, 0, len(dbUsers))
		for _, u := range dbUsers {
			pbUsers = append(pbUsers, &user.User{
				Id:    int32(u.ID),
				Name:  u.Name,
				Age:  int32(u.Age),
				Location:  safeString(u.Location),
				Email: safeString(u.Email),
			})
		}

	return &user.GetResponse{Users: pbUsers}, nil
}
func (s *userServer) New(ctx context.Context, req *user.PostRequest) (*user.SuccessResponse, error) {
	return &user.SuccessResponse{Success: true}, nil
}

func connectDB() (*db.Queries, *sql.DB) {
	// Load configs
		dbHost := os.Getenv("DB_HOST")
		dbPort := os.Getenv("DB_PORT")
		dbUser := os.Getenv("DB_USER")
		dbPass := os.Getenv("DB_PASS")
		dbName := os.Getenv("DB_NAME")
		dsn := fmt.Sprintf("%s:%s@tcp(%s:%s)/%s", dbUser, dbPass, dbHost, dbPort, dbName)

	// Open connection pool
		sqlDB, err := sql.Open("mysql", dsn)
		if err != nil {
			log.Fatal(err)
		}

	// Verify connection
		if err = sqlDB.Ping(); err != nil {
			log.Fatal(err)
		}

	// Wrap with sqlc, then return
		return db.New(sqlDB), sqlDB
}

func main(){
	// Connect to DB
		query, sqlDB := connectDB()
		defer sqlDB.Close()

	// Listen to port 50051
		lis, err := net.Listen("tcp", ":50051")
		if err != nil {
			log.Fatalf("failed to listen: %v", err)
		}

		grpcServer := grpc.NewServer()
		user.RegisterUserServiceServer(grpcServer, &userServer{ query:query })

		log.Println("Go service is running on :50051")
		if err := grpcServer.Serve(lis); err != nil {
			log.Fatalf("failed to serve: %v", err)
		}
}