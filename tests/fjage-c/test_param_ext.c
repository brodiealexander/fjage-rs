#include "fjage-rs.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

void test_assert(const char *name, int pass);

void test_param_set_read_only(fjage_gw_t gw, fjage_aid_t aid)
{
    test_assert("set param (-read-only string)", fjage_param_set_string(gw, aid, "roStringParam", "dummy", -1) == -1);
    test_assert("set param (-read-only int)", fjage_param_set_int(gw, aid, "roIntParam", 0, -1) == -1);
    test_assert("set param (-read-only long)", fjage_param_set_long(gw, aid, "roLongParam", 0, -1) == -1);
    test_assert("set param (-read-only float)", fjage_param_set_float(gw, aid, "roFloatParam", 0.0, -1) == -1);

    // add cases for other variables
}

void test_param_array_write(fjage_gw_t gw, fjage_aid_t aid)
{
    bool rwBoolArrayParam[4] = {false, true, true, false};
    int rwIntArrayParam[1] = {20};
    long rwLongArrayParam[2] = {1920, 1080};
    float rwFloatArrayParam[3] = {27.0f, -26.0f, 25.0f};
    double rwDoubleArrayParam[3] = {-10.0f, 5.0f, 30.0f};
    char **rwStringArrayParam = malloc(sizeof(void *) * 2);
    char string1[] = "Hello";
    char string2[] = "World";
    rwStringArrayParam[0] = string1;
    rwStringArrayParam[1] = string2;
    test_assert("set param (+int[1])", fjage_param_set_int_array(gw, aid, "rwIntArrayParam", rwIntArrayParam, 1, -1) == 0);
    test_assert("set param (+long[2])", fjage_param_set_long_array(gw, aid, "rwLongArrayParam", rwLongArrayParam, 2, -1) == 0);
    test_assert("set param (+float[3])", fjage_param_set_float_array(gw, aid, "rwFloatArrayParam", rwFloatArrayParam, 3, -1) == 0);
    test_assert("set param (+double[3])", fjage_param_set_double_array(gw, aid, "rwDoubleArrayParam", rwDoubleArrayParam, 3, -1) == 0);
    test_assert("set param (+string[2])", fjage_param_set_string_array(gw, aid, "rwStringArrayParam", rwStringArrayParam, 2, -1) == 0);
    free(rwStringArrayParam);
}

void test_param_array_read(fjage_gw_t gw, fjage_aid_t aid)
{
    bool rwBoolArrayParam[4];
    int rwIntArrayParam[1];
    long rwLongArrayParam[2];
    float rwFloatArrayParam[3];
    double rwDoubleArrayParam[3];
    char **rwStringArrayParam = malloc(sizeof(void *) * 2);
    printf("TRUTH: %d %d %d\n", rwBoolArrayParam[0], rwBoolArrayParam[1], rwBoolArrayParam[2]);
    test_assert("get param (+int[1])", fjage_param_get_int_array(gw, aid, "rwIntArrayParam", rwIntArrayParam, 1, -1) == 1 && rwIntArrayParam[0] == 20);
    test_assert("get param (+long[2])", fjage_param_get_long_array(gw, aid, "rwLongArrayParam", rwLongArrayParam, 2, -1) == 2 && rwLongArrayParam[0] == 1920);
    test_assert("get param (+float[3])", fjage_param_get_float_array(gw, aid, "rwFloatArrayParam", rwFloatArrayParam, 3, -1) == 3 && rwFloatArrayParam[0] == 27.0f);
    test_assert("get param (+double[3])", fjage_param_get_double_array(gw, aid, "rwDoubleArrayParam", rwDoubleArrayParam, 3, -1) == 3 && rwDoubleArrayParam[0] == -10.0f && rwDoubleArrayParam[1] == 5.0f && rwDoubleArrayParam[2] == 30.0f);
    test_assert("get param (+string[2])", fjage_param_get_string_array(gw, aid, "rwStringArrayParam", rwStringArrayParam, 2, -1) == 2 && strcmp(rwStringArrayParam[0], "Hello") == 0 && strcmp(rwStringArrayParam[1], "World") == 0);
    free(rwStringArrayParam[0]);
    free(rwStringArrayParam[1]);
    free(rwStringArrayParam);
}

void test_param_ext(fjage_gw_t gw)
{

    fjage_aid_t aid = fjage_aid_create("gwtestalpha");

    test_param_array_write(gw, aid);
    test_param_array_read(gw, aid);

    test_assert("set param (+string)", fjage_param_set_string(gw, aid, "rwStringParam", "dummy", -1) == 0);
    test_assert("set param (+int)", fjage_param_set_int(gw, aid, "rwIntParam", 0, -1) == 0);
    test_assert("set param (+long)", fjage_param_set_long(gw, aid, "rwLongParam", 0, -1) == 0);
    test_assert("set param (+float)", fjage_param_set_float(gw, aid, "rwFloatParam", 0.0, -1) == 0);
    test_assert("set param (+double)", fjage_param_set_double(gw, aid, "rwDoubleParam", 0.0, -1) == 0);

    fjage_aid_destroy(aid);
}