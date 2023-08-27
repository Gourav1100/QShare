#include <iostream>
#include <cstring>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>

int main() {
    int clientSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (clientSocket == -1) {
        std::cerr << "Failed to create socket." << std::endl;
        return 1;
    }

    sockaddr_in serverAddr{};
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(8080);  // Replace with the port your Rust server is listening on
    inet_pton(AF_INET, "127.0.0.1", &(serverAddr.sin_addr));

    if (connect(clientSocket, (struct sockaddr*)&serverAddr, sizeof(serverAddr)) == -1) {
        std::cerr << "Connection failed." << std::endl;
        return 1;
    }

    std::string message = "Hello from C++ client!";
    send(clientSocket, message.c_str(), message.size(), 0);

    char buffer[1024] = {0};
    recv(clientSocket, buffer, sizeof(buffer), 0);
    std::cout << "Server says: " << buffer << std::endl;

    close(clientSocket);
    return 0;
}
