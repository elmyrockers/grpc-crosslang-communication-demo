package controller


import (
	"github.com/gofiber/fiber/v2"
	"fmt"
	// "github.com/davecgh/go-spew/spew"

	"context"
	// "google.golang.org/grpc"
	"github.com/elmyrockers/grpc-crosslang-communication-demo/http-server/pb/user"
)

type UserController struct {
	Client user.UserServiceClient
}



// Web routes
	func (u *UserController) List( c *fiber.Ctx ) error {
		return c.Render( "index", fiber.Map{
			"title": "gRPC Demo",
		})
	}



// API routes
	func (u *UserController) All( c *fiber.Ctx ) error {
		response, err := u.Client.All(context.Background(), &user.GetRequest{})

		if err != nil {
			return c.Status(500).JSON(fiber.Map{
				"success": false,
				"error":   err.Error(),
			})
		}
		return c.JSON( response.Users )
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