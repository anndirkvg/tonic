ghz --insecure --proto ./examples/proto/echo/echo.proto -n 100000 -c 10 '[::1]:50051' --call grpc.examples.echo.Echo.BidirectionalStreamingEcho
