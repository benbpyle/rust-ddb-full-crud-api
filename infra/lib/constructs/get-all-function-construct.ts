import { Construct } from "constructs";
import { FunctionProps } from "../data-interaces/function-props";
import { RustFunction } from "cargo-lambda-cdk";
import { LambdaIntegration } from "aws-cdk-lib/aws-apigateway";

export class GetAllFunctionConstruct extends Construct {

    constructor(scope: Construct, id: string, props: FunctionProps) {
        super(scope, id);

        const func = new RustFunction(scope, 'GetAllFunction', {
            functionName: 'sample-get-all',
            manifestPath: 'lambdas/get-all',
            memorySize: 256,
            environment: {
                "TABLE_NAME": props.table.tableName
            }
        })

        props.resource
            .addMethod('GET', new LambdaIntegration(
                func, {
                proxy: true
            }));

        props.table.grantReadData(func);
    }
}