import {Construct} from "constructs";
import {FunctionProps} from "../data-interaces/function-props";
import {RustFunction} from "cargo-lambda-cdk";
import {LambdaIntegration} from "aws-cdk-lib/aws-apigateway";

export class GetByIdFunctionConstruct extends Construct {

    constructor(scope: Construct, id: string, props: FunctionProps) {
        super(scope, id);

        const func = new RustFunction(scope, 'GetByIdFunction', {
            functionName: 'sample-get-by-id',
            manifestPath: 'lambdas/get-by-id',
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