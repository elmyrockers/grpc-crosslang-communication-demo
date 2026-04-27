package main

import (
	// "fmt"
	"log"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/http-server/controller"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/jet/v2"
)

func main() {
	// Create app with Jet template engine
		engine := jet.New("./views", ".jet")
		app := fiber.New(fiber.Config{
			Views: engine,
		})
	
	// Web Routes
		app.Get("/", func(c *fiber.Ctx) error {
			return c.Redirect("/users")
		})
		userController := controller.UserController{}
		app.Get("/users", userController.List )

	// API Routes
		api := app.Group( "/api" )
		api.Get("/users", userController.All )

	log.Fatal(app.Listen(":3000"))
}