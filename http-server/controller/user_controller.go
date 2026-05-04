package controller


import (
	"github.com/gofiber/fiber/v2"
	"fmt"
	// "github.com/davecgh/go-spew/spew"
)

type User struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Age   int `json:"age"`
	Location string `json:"location"`
	Email string `json:"email"`
}
type UserController struct {}



// Web routes
	func (u *UserController) List( c *fiber.Ctx ) error {
		return c.Render( "index", fiber.Map{
			"title": "gRPC Demo",
		})
	}



// API routes
	func (u *UserController) All( c *fiber.Ctx ) error {
		return c.JSON([]User{
			{ID: 1, Name: "Helmi Aziz", Age: 27, Location: "Kuala Lumpur", Email: "helmi@xeno.com.my"},
			{ID: 2, Name: "Akmal Hazim", Age: 30, Location: "Alor Setar", Email: "hazim@gmail.com"},
		})
	}

	func (u *UserController) New( c *fiber.Ctx ) error {
		// Get form values
			payload := struct {
				Name  string `json:"name"`
				Email string `json:"email"`
				Age string `json:"age"`
				Location string `json:"location"`
			}{}
			err := c.BodyParser(&payload);
			if err != nil {
				return err
			}

		// Return response
			return c.JSON(fiber.Map{
				"success": err==nil,
				"payload": payload,
			})
	}

	func (u *UserController) Edit( c *fiber.Ctx ) error {
		fmt.Println( "Edit User" )

		return c.JSON(fiber.Map{
			"success": true,
		})
	}

	func (u *UserController) Delete( c *fiber.Ctx ) error {
		fmt.Println( "Delete User" )

		return c.JSON(fiber.Map{
			"success": true,
		})
	}