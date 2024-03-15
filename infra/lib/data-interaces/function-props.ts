import {IResource, RestApi} from "aws-cdk-lib/aws-apigateway";
import {ITableV2} from "aws-cdk-lib/aws-dynamodb";

export interface FunctionProps {
    api: RestApi;
    resource: IResource
    table: ITableV2
}