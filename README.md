# A Proven and Comprehensive Pattern for Building an API with Rust and Lambda

Purpose: Supports the article on [Binaryheap.com](https://www.binaryheap.com/api-with-rust-and-lambda/)

### Dependencies

-   Rust
-   Node
-   CDK

### Deploying

Being that this is a CDK project, deploying the API with Rust and Lambda requires running this command from the terminal.

```bash
cdk deploy
```

I've built this with a single stack that'll deploy the Lambdas, DynamoDB Table, and API Gateway.

### Running the API

Once deployed, visit the AWS Console and find the AWS-assigned URL to your new API Gateway.  With that value, you can load Postman and launch the included Postman collection.  It has a variable named `API_ENDPOINT` which is where the assigned URL needs to go.  

My recommendation is you start with the POST endpoint, create some new items, and then explore from there.

### Clean up

The last piece of this is that when you are done, just run `cdk destroy` in the project directory and everything will clean up. 

