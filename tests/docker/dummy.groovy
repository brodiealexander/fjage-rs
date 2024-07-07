import org.arl.fjage.*
import org.arl.fjage.remote.*
import org.arl.fjage.shell.*
import org.arl.fjage.connectors.*

import org.arl.fjage.param.*

// Groovy will automatically generate setters and getters per fjage docs.
enum StaticTestParams implements Parameter{
    roIntParam,
    roLongParam,
    roFloatParam,
    roStringParam,
    rwBoolParam,
    rwIntParam,
    rwLongParam,
    rwFloatParam,
    rwDoubleParam,
    rwStringParam,
    rwIntArrayParam,
    rwLongArrayParam,
    rwFloatArrayParam,
    rwDoubleArrayParam,
    rwStringArrayParam,
}

class GatewayTestAgent extends org.arl.fjage.Agent {
    final int roIntParam = 42
    final long roLongParam = 50
    final float roFloatParam = 20.99
    final String roStringParam = "FINAL STRING"
    boolean rwBoolParam = false
    int rwIntParam = 72
    long rwLongParam = 99
    float rwFloatParam = 277.76
    double rwDoubleParam = 79.99
    String rwStringParam = "MUTABLE STRING"
    int[] rwIntArrayParam = [27]
    long[] rwLongArrayParam = [640, 480]
    float[] rwFloatArrayParam = [27.0, -26.0, 25.0]
    double[] rwDoubleArrayParam = [0.0, 0.0, 15.0]
    String[] rwStringArrayParam = ["World", "Hello"]
    // This is an analog of node.location in UnetStack
    

    void init() {
        add new ParameterMessageBehavior(StaticTestParams)
        register 'org.arl.fjage.test.Services.PARAMETER_TEST'
    }
} 

container.add 'gwtestalpha', new GatewayTestAgent()
container.add 'gwtestbeta', new GatewayTestAgent()