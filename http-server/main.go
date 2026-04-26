package main

import (
	// "fmt"
	"log"
	"github.com/elmyrockers/grpc-crosslang-in-action/http-server/controller"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/jet/v2"
)

func main() {
	// Create app with Jet template engine
		engine := jet.New("./views", ".jet")
		app := fiber.New(fiber.Config{
			Views: engine,
		})
	
	// Routes
		app.Get("/", func(c *fiber.Ctx) error {
			return c.Redirect("/users")
		})

		userController := controller.UserController{}
		app.Get("/users", userController.List )
		app.Get("/users/new", userController.New )

	log.Fatal(app.Listen(":3000"))
}