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
        {Id: 1, Name: "Helmi", Age: 30, Location: "Kuala Lumpur", Email: "helmi@example.com"},
        {Id: 2, Name: "Nasrul", Age: 28, Location: "Selangor", Email: "nasrul@example.com"},
    }
    return &user.GetResponse{Users: users}, nil
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