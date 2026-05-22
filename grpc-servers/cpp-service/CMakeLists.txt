cmake_minimum_required(VERSION 3.28)
project(cpp-service LANGUAGES CXX)


# Define executable
	add_executable(cpp-service)
	target_compile_features(cpp-service PRIVATE cxx_std_23)
	target_sources(cpp-service
		PRIVATE
			src/main.cpp
			pb/user.grpc.pb.cc
			pb/user.pb.cc
		PRIVATE FILE_SET CXX_MODULES
		BASE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/src
		FILES
			src/grpc.service.cppm
	)
	target_include_directories(cpp-service
		PUBLIC 
			${CMAKE_CURRENT_SOURCE_DIR}/src
			${CMAKE_CURRENT_SOURCE_DIR}/pb
	)


# gRPC libraries - include protobuf and flatbuffers
	find_package(gRPC CONFIG REQUIRED)
	target_link_libraries(cpp-service PRIVATE gRPC::gpr gRPC::grpc gRPC::grpc++ gRPC::grpc++_alts)
	find_package(protobuf CONFIG REQUIRED)
	target_link_libraries(cpp-service PRIVATE protobuf::libupb protobuf::libprotoc protobuf::libprotobuf protobuf::libprotobuf-lite)
	find_package(flatbuffers CONFIG REQUIRED)
	target_link_libraries(cpp-service PRIVATE flatbuffers::flatbuffers)

# For debugging purpose - boost-pfr
	find_package(Boost REQUIRED COMPONENTS pfr)
	target_link_libraries(cpp-service PRIVATE Boost::pfr)





set_target_properties(cpp-service PROPERTIES OUTPUT_NAME "app")