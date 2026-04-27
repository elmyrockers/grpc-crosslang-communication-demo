package controller


import (
	"github.com/gofiber/fiber/v2"

	"fmt"
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
			{ID: 2, Name: "Hazim", Age: 30, Location: "Alor Setar", Email: "hazim@gmail.com"},
		})
	}

	func (u *UserController) New( c *fiber.Ctx ) error {
		fmt.Println( "New user" )
		return nil
	}

	func (u *UserController) Edit( c *fiber.Ctx ) error {
		fmt.Println( "Edit User" )
		return nil
	}

	func (u *UserController) Delete( c *fiber.Ctx ) error {
		fmt.Println( "Delete User" )
		return nil
	}