import {Construct} from "constructs";
import {FunctionProps} from "../data-interaces/function-props";
import {RustFunction} from "cargo-lambda-cdk";
import {LambdaIntegration} from "aws-cdk-lib/aws-apigateway";

export class PostFunctionConstruct extends Construct {
    constructor(scope: Construct, id: string, props: FunctionProps) {
        super(scope, id);

        const func = new RustFunction(scope, 'PostFunction', {
            functionName: 'sample-post',
            manifestPath: 'lambdas/post',
            memorySize: 256,
            environment: {
                "TABLE_NAME": props.table.tableName
            }
        })

        props.resource
            .addMethod('POST', new LambdaIntegration(
                func, {
                    proxy: true
                }));

        props.table.grantWriteData(func);
    }
}