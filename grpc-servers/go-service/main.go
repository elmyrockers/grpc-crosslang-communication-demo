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