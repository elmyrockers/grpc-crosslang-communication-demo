package main


import (
	"context"
	"log"
	
	"net"
	"google.golang.org/grpc"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/grpc-servers/go-service/pb/user"
)


type userServer struct {
	user.UnimplementedUserServiceServer
}
func (s *userServer) All(ctx context.Context, req *user.GetRequest) (*user.GetResponse, error) {
	users := []*user.User{
		{Id: 1, Name: "Helmi Aziz", Age: 27, Location: "Kuala Lumpur", Email: "helmi@xeno.com.my"},
		{Id: 2, Name: "Akmal Hazim", Age: 30, Location: "Alor Setar", Email: "hazim@gmail.com"},
	}
	return &user.GetResponse{Users: users}, nil
}
func (s *userServer) New(ctx context.Context, req *user.PostRequest) (*user.SuccessResponse, error) {
	return &user.SuccessResponse{Success: true}, nil
}

func connectDB() *db.Queries {
	// Open connection pool
		sqlDB, err := sql.Open("mysql", "root:12345@tcp(localhost:3306)/grpc-demo")
		if err != nil {
			log.Fatal(err)
		}
		defer sqlDB.Close()

	// Verify connection
		if err = sqlDB.Ping(); err != nil {
			log.Fatal(err)
		}

	// Wrap with sqlc, then return
		return db.New(sqlDB)
}


func main(){
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	user.RegisterUserServiceServer(grpcServer, &userServer{})

	log.Println("Go service is running on :50051")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}