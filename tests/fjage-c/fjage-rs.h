#include "fjage.h"

// New to fjage-rs
// double param extension
int fjage_param_set_double(fjage_gw_t gw, fjage_aid_t aid, const char *param, double value, int ndx);
double fjage_param_get_double(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, double defval);

// array params extension
// setters
int fjage_param_set_int_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, int *value, int len, int ndx);
int fjage_param_set_long_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, long *value, int len, int ndx);
int fjage_param_set_float_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, float *value, int len, int ndx);
int fjage_param_set_double_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, double *value, int len, int ndx);
int fjage_param_set_string_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, const char **value, int len, int ndx);

// getters
int fjage_param_get_int_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, int *value, int maxlen, int ndx);
int fjage_param_get_long_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, long *value, int maxlen, int ndx);
int fjage_param_get_float_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, float *value, int maxlen, int ndx);
int fjage_param_get_double_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, double *value, int maxlen, int ndx);
int fjage_param_get_string_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, char **value, int maxlen, int ndx);