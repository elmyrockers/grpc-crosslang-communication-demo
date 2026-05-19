module;
#include <print>

export module grpc.service;

export class UserServer
{
public:
	UserServer(){
		std::print( "UserServer dilaksanakan!" );
	}
	~UserServer(){

	}
	
};