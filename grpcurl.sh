grpcurl -vv -plaintext -import-path ./examples/proto/echo -proto echo.proto  '[::1]:50051' grpc.examples.echo.Echo.BidirectionalStreamingEcho
