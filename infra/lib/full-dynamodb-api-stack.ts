import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { ApiGatewayConstruct } from "./constructs/api-gateway-construct";
import { GetByIdFunctionConstruct } from "./constructs/get-by-id-function-construct";
import { TableConstruct } from "./constructs/table-construct";
import { PostFunctionConstruct } from "./constructs/post-function-construct";
import { DeleteByIdFunctionConstruct } from './constructs/delete-by-id-function-construct';
import { PutByIdFunctionConstruct } from './constructs/put-by-id-function-construct';

// import * as sqs from 'aws-cdk-shared/aws-sqs';

export class FullDynamodbApiStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const api = new ApiGatewayConstruct(this, 'ApiGatewayConstruct');
        const table = new TableConstruct(this, 'TableConstruct');
        new GetByIdFunctionConstruct(this, 'GetByIdFunctionConstruct',
            {
                resource: api.idResource,
                api: api.api,
                table: table.table
            });
        new PostFunctionConstruct(this, 'PostFunctionConstruct',
            {
                resource: api.topResource,
                api: api.api,
                table: table.table
            }
        );
        new DeleteByIdFunctionConstruct(this, 'DeleteByIdFunctionConstruct',
            {
                resource: api.idResource,
                api: api.api,
                table: table.table
            }
        );
        new PutByIdFunctionConstruct(this, 'PutByIdFunctionConstruct',
            {
                resource: api.idResource,
                api: api.api,
                table: table.table
            }
        );
    }
}
