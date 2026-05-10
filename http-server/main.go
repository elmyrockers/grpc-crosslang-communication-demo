package main

import (
	// "fmt"
	// "github.com/davecgh/go-spew/spew"

	"log"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/http-server/controller"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/jet/v2"

	"google.golang.org/grpc"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/http-server/pb/user"
)

func main() {
	// Connect to go-service (gRPC server)
		connection, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
		if err != nil {
			log.Fatalf("did not connect: %v", err)
		}
		defer connection.Close()
	
	// Set gRPC Client as UserController attribute
		client := user.NewUserServiceClient(connection)
		userController := controller.UserController{ Client: client }

	// Create app with Jet template engine
		engine := jet.New("./views", ".jet")
		app := fiber.New(fiber.Config{
			Views: engine,
		})
	
	// Web Routes
		app.Get("/", func(c *fiber.Ctx) error {
			return c.Redirect("/users")
		})
		app.Get("/users", userController.List )

	// API Routes
		api := app.Group( "/api" )
		api.Get("/users", userController.All )
		api.Post("/users", userController.New )
		api.Patch("/users/:id", userController.Edit )
		api.Delete("/users/:id", userController.Delete )

	log.Fatal(app.Listen(":3000"))
}