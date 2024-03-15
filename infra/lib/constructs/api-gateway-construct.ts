import {Construct} from "constructs";
import {RestApi, IResource, Resource} from "aws-cdk-lib/aws-apigateway";


export class ApiGatewayConstruct extends Construct {
    private readonly _api: RestApi;
    private readonly _idResource: Resource;

    constructor(scope: Construct, id: string) {
        super(scope, id);
        this._api = new RestApi(this,
            'RestApi', {
                description: 'Sample API',
                restApiName: 'Sample API',
                disableExecuteApiEndpoint: false,
                deployOptions: {
                    stageName: `main`,
                },
            });

        this._idResource = new Resource(this, 'TopResource', {
            parent: this._api.root,
            pathPart: "{id}"
        });
    }

    get idResource(): IResource {
        return this._idResource;
    }

    get topResource(): IResource {
        return this._api.root;
    }
    get api(): RestApi {
        return this._api;
    }
}