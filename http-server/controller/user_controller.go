package controller


import (
	"github.com/gofiber/fiber/v2"

	"fmt"
)

type UserController struct {

}

func (u *UserController) List( c *fiber.Ctx ) error {
	fmt.Println( "List of users" )
	return nil
}

func (u *UserController) New( c *fiber.Ctx ) error {
	fmt.Println( "New user" )
	return nil
}