package main

import (
	// "fmt"
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()
	
	// Routes
		app.Get("/", func(c *fiber.Ctx) error {
			return c.Redirect("/users")
		})
		app.Get("/users", func(c *fiber.Ctx) error {
			return c.SendString( "test" )
		})

	log.Fatal(app.Listen(":3000"))
	// fmt.Println( "test sahaja" )
}