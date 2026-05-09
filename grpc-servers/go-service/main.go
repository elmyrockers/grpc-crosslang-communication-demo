package main


import (
	"context"
	"log"
	
	"net"
	"google.golang.org/grpc"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/grpc-servers/go-service/pb/user"
)









func main(){
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	user.RegisterUserServiceServer(grpcServer, &userServer{})

	log.Println("gRPC server running on :50051")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}