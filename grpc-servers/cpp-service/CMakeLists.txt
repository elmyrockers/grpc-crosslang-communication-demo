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




# For debugging purpose - dbg-macro and boost-pfr
	find_path(DBG_MACRO_INCLUDE_DIRS "dbg-macro/dbg.h")
	target_include_directories(cpp-service PRIVATE ${DBG_MACRO_INCLUDE_DIRS})
	find_package(Boost REQUIRED COMPONENTS pfr)
	target_link_libraries(cpp-service PRIVATE Boost::pfr)


set_target_properties(cpp-service PROPERTIES OUTPUT_NAME "app")