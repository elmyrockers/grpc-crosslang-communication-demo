cmake_minimum_required(VERSION 3.28)
project(cpp-service LANGUAGES CXX)


# Define executable
	add_executable(cpp-service)
	target_compile_features(cpp-service PRIVATE cxx_std_23)
	target_sources(cpp-service
			PRIVATE
				src/main.cpp
			PUBLIC
				FILE_SET CXX_MODULES FILES
					src/grpc.service.cppm
		)
	target_include_directories(cpp-service
		PUBLIC 
			${CMAKE_CURRENT_SOURCE_DIR}/src
	)














set_target_properties(cpp-service PROPERTIES OUTPUT_NAME "app")