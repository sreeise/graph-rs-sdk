// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorkbookFunctionsApiClient,
    ResourceIdentity::WorkbookFunctions
);

impl WorkbookFunctionsApiClient {
    delete!(
        doc: "Delete navigation property functions for drives",
        name: delete_functions,
        path: "/functions"
    );
    get!(
        doc: "Get functions from drives",
        name: get_functions,
        path: "/functions"
    );
    patch!(
        doc: "Update the navigation property functions in drives",
        name: update_functions,
        path: "/functions",
        body: true
    );
    post!(
        doc: "Invoke action abs",
        name: abs,
        path: "/functions/abs",
        body: true
    );
    post!(
        doc: "Invoke action accrInt",
        name: accr_int,
        path: "/functions/accrInt",
        body: true
    );
    post!(
        doc: "Invoke action accrIntM",
        name: accr_int_m,
        path: "/functions/accrIntM",
        body: true
    );
    post!(
        doc: "Invoke action acos",
        name: acos,
        path: "/functions/acos",
        body: true
    );
    post!(
        doc: "Invoke action acosh",
        name: acosh,
        path: "/functions/acosh",
        body: true
    );
    post!(
        doc: "Invoke action acot",
        name: acot,
        path: "/functions/acot",
        body: true
    );
    post!(
        doc: "Invoke action acoth",
        name: acoth,
        path: "/functions/acoth",
        body: true
    );
    post!(
        doc: "Invoke action amorDegrc",
        name: amor_degrc,
        path: "/functions/amorDegrc",
        body: true
    );
    post!(
        doc: "Invoke action amorLinc",
        name: amor_linc,
        path: "/functions/amorLinc",
        body: true
    );
    post!(
        doc: "Invoke action and",
        name: and,
        path: "/functions/and",
        body: true
    );
    post!(
        doc: "Invoke action arabic",
        name: arabic,
        path: "/functions/arabic",
        body: true
    );
    post!(
        doc: "Invoke action areas",
        name: areas,
        path: "/functions/areas",
        body: true
    );
    post!(
        doc: "Invoke action asc",
        name: asc,
        path: "/functions/asc",
        body: true
    );
    post!(
        doc: "Invoke action asin",
        name: asin,
        path: "/functions/asin",
        body: true
    );
    post!(
        doc: "Invoke action asinh",
        name: asinh,
        path: "/functions/asinh",
        body: true
    );
    post!(
        doc: "Invoke action atan",
        name: atan,
        path: "/functions/atan",
        body: true
    );
    post!(
        doc: "Invoke action atan2",
        name: functions,
        path: "/functions/atan2",
        body: true
    );
    post!(
        doc: "Invoke action atanh",
        name: atanh,
        path: "/functions/atanh",
        body: true
    );
    post!(
        doc: "Invoke action aveDev",
        name: ave_dev,
        path: "/functions/aveDev",
        body: true
    );
    post!(
        doc: "Invoke action average",
        name: average,
        path: "/functions/average",
        body: true
    );
    post!(
        doc: "Invoke action averageA",
        name: average_a,
        path: "/functions/averageA",
        body: true
    );
    post!(
        doc: "Invoke action averageIf",
        name: average_if,
        path: "/functions/averageIf",
        body: true
    );
    post!(
        doc: "Invoke action averageIfs",
        name: average_ifs,
        path: "/functions/averageIfs",
        body: true
    );
    post!(
        doc: "Invoke action bahtText",
        name: baht_text,
        path: "/functions/bahtText",
        body: true
    );
    post!(
        doc: "Invoke action base",
        name: base,
        path: "/functions/base",
        body: true
    );
    post!(
        doc: "Invoke action besselI",
        name: bessel_i,
        path: "/functions/besselI",
        body: true
    );
    post!(
        doc: "Invoke action besselJ",
        name: bessel_j,
        path: "/functions/besselJ",
        body: true
    );
    post!(
        doc: "Invoke action besselK",
        name: bessel_k,
        path: "/functions/besselK",
        body: true
    );
    post!(
        doc: "Invoke action besselY",
        name: bessel_y,
        path: "/functions/besselY",
        body: true
    );
    post!(
        doc: "Invoke action beta_Dist",
        name: beta_dist,
        path: "/functions/beta_Dist",
        body: true
    );
    post!(
        doc: "Invoke action beta_Inv",
        name: beta_inv,
        path: "/functions/beta_Inv",
        body: true
    );
    post!(
        doc: "Invoke action bin2Dec",
        name: bin_2_dec,
        path: "/functions/bin2Dec",
        body: true
    );
    post!(
        doc: "Invoke action bin2Hex",
        name: bin_2_hex,
        path: "/functions/bin2Hex",
        body: true
    );
    post!(
        doc: "Invoke action bin2Oct",
        name: bin_20_ct,
        path: "/functions/bin2Oct",
        body: true
    );
    post!(
        doc: "Invoke action binom_Dist",
        name: binom_dist,
        path: "/functions/binom_Dist",
        body: true
    );
    post!(
        doc: "Invoke action binom_Dist_Range",
        name: binom_dist_range,
        path: "/functions/binom_Dist_Range",
        body: true
    );
    post!(
        doc: "Invoke action binom_Inv",
        name: binom_inv,
        path: "/functions/binom_Inv",
        body: true
    );
    post!(
        doc: "Invoke action bitand",
        name: bitand,
        path: "/functions/bitand",
        body: true
    );
    post!(
        doc: "Invoke action bitlshift",
        name: bitlshift,
        path: "/functions/bitlshift",
        body: true
    );
    post!(
        doc: "Invoke action bitor",
        name: bitor,
        path: "/functions/bitor",
        body: true
    );
    post!(
        doc: "Invoke action bitrshift",
        name: bitrshift,
        path: "/functions/bitrshift",
        body: true
    );
    post!(
        doc: "Invoke action bitxor",
        name: bitxor,
        path: "/functions/bitxor",
        body: true
    );
    post!(
        doc: "Invoke action ceiling_Math",
        name: ceiling_math,
        path: "/functions/ceiling_Math",
        body: true
    );
    post!(
        doc: "Invoke action ceiling_Precise",
        name: ceiling_precise,
        path: "/functions/ceiling_Precise",
        body: true
    );
    post!(
        doc: "Invoke action char",
        name: char,
        path: "/functions/char",
        body: true
    );
    post!(
        doc: "Invoke action chiSq_Dist",
        name: chi_sq_dist,
        path: "/functions/chiSq_Dist",
        body: true
    );
    post!(
        doc: "Invoke action chiSq_Dist_RT",
        name: chi_sq_dist_rt,
        path: "/functions/chiSq_Dist_RT",
        body: true
    );
    post!(
        doc: "Invoke action chiSq_Inv",
        name: chi_sq_inv,
        path: "/functions/chiSq_Inv",
        body: true
    );
    post!(
        doc: "Invoke action chiSq_Inv_RT",
        name: chi_sq_inv_rt,
        path: "/functions/chiSq_Inv_RT",
        body: true
    );
    post!(
        doc: "Invoke action choose",
        name: choose,
        path: "/functions/choose",
        body: true
    );
    post!(
        doc: "Invoke action clean",
        name: clean,
        path: "/functions/clean",
        body: true
    );
    post!(
        doc: "Invoke action code",
        name: code,
        path: "/functions/code",
        body: true
    );
    post!(
        doc: "Invoke action columns",
        name: columns,
        path: "/functions/columns",
        body: true
    );
    post!(
        doc: "Invoke action combin",
        name: combin,
        path: "/functions/combin",
        body: true
    );
    post!(
        doc: "Invoke action combina",
        name: combina,
        path: "/functions/combina",
        body: true
    );
    post!(
        doc: "Invoke action complex",
        name: complex,
        path: "/functions/complex",
        body: true
    );
    post!(
        doc: "Invoke action concatenate",
        name: concatenate,
        path: "/functions/concatenate",
        body: true
    );
    post!(
        doc: "Invoke action confidence_Norm",
        name: confidence_norm,
        path: "/functions/confidence_Norm",
        body: true
    );
    post!(
        doc: "Invoke action confidence_T",
        name: confidence_t,
        path: "/functions/confidence_T",
        body: true
    );
    post!(
        doc: "Invoke action convert",
        name: convert,
        path: "/functions/convert",
        body: true
    );
    post!(
        doc: "Invoke action cos",
        name: cos,
        path: "/functions/cos",
        body: true
    );
    post!(
        doc: "Invoke action cosh",
        name: cosh,
        path: "/functions/cosh",
        body: true
    );
    post!(
        doc: "Invoke action cot",
        name: cot,
        path: "/functions/cot",
        body: true
    );
    post!(
        doc: "Invoke action coth",
        name: coth,
        path: "/functions/coth",
        body: true
    );
    post!(
        doc: "Invoke action count",
        name: count,
        path: "/functions/count",
        body: true
    );
    post!(
        doc: "Invoke action countA",
        name: count_a,
        path: "/functions/countA",
        body: true
    );
    post!(
        doc: "Invoke action countBlank",
        name: count_blank,
        path: "/functions/countBlank",
        body: true
    );
    post!(
        doc: "Invoke action countIf",
        name: count_if,
        path: "/functions/countIf",
        body: true
    );
    post!(
        doc: "Invoke action countIfs",
        name: count_ifs,
        path: "/functions/countIfs",
        body: true
    );
    post!(
        doc: "Invoke action coupDayBs",
        name: coup_day_bs,
        path: "/functions/coupDayBs",
        body: true
    );
    post!(
        doc: "Invoke action coupDays",
        name: coup_days,
        path: "/functions/coupDays",
        body: true
    );
    post!(
        doc: "Invoke action coupDaysNc",
        name: coup_days_nc,
        path: "/functions/coupDaysNc",
        body: true
    );
    post!(
        doc: "Invoke action coupNcd",
        name: coup_ncd,
        path: "/functions/coupNcd",
        body: true
    );
    post!(
        doc: "Invoke action coupNum",
        name: coup_num,
        path: "/functions/coupNum",
        body: true
    );
    post!(
        doc: "Invoke action coupPcd",
        name: coup_pcd,
        path: "/functions/coupPcd",
        body: true
    );
    post!(
        doc: "Invoke action csc",
        name: csc,
        path: "/functions/csc",
        body: true
    );
    post!(
        doc: "Invoke action csch",
        name: csch,
        path: "/functions/csch",
        body: true
    );
    post!(
        doc: "Invoke action cumIPmt",
        name: cum_i_pmt,
        path: "/functions/cumIPmt",
        body: true
    );
    post!(
        doc: "Invoke action cumPrinc",
        name: cum_princ,
        path: "/functions/cumPrinc",
        body: true
    );
    post!(
        doc: "Invoke action date",
        name: date,
        path: "/functions/date",
        body: true
    );
    post!(
        doc: "Invoke action datevalue",
        name: datevalue,
        path: "/functions/datevalue",
        body: true
    );
    post!(
        doc: "Invoke action daverage",
        name: daverage,
        path: "/functions/daverage",
        body: true
    );
    post!(
        doc: "Invoke action day",
        name: day,
        path: "/functions/day",
        body: true
    );
    post!(
        doc: "Invoke action days",
        name: days,
        path: "/functions/days",
        body: true
    );
    post!(
        doc: "Invoke action days360",
        name: days_360,
        path: "/functions/days360",
        body: true
    );
    post!(
        doc: "Invoke action db",
        name: db,
        path: "/functions/db",
        body: true
    );
    post!(
        doc: "Invoke action dbcs",
        name: dbcs,
        path: "/functions/dbcs",
        body: true
    );
    post!(
        doc: "Invoke action dcount",
        name: dcount,
        path: "/functions/dcount",
        body: true
    );
    post!(
        doc: "Invoke action dcountA",
        name: dcount_a,
        path: "/functions/dcountA",
        body: true
    );
    post!(
        doc: "Invoke action ddb",
        name: ddb,
        path: "/functions/ddb",
        body: true
    );
    post!(
        doc: "Invoke action dec2Bin",
        name: dec_2b_in,
        path: "/functions/dec2Bin",
        body: true
    );
    post!(
        doc: "Invoke action dec2Hex",
        name: dec_2_hex,
        path: "/functions/dec2Hex",
        body: true
    );
    post!(
        doc: "Invoke action dec2Oct",
        name: dec_20_ct,
        path: "/functions/dec2Oct",
        body: true
    );
    post!(
        doc: "Invoke action decimal",
        name: decimal,
        path: "/functions/decimal",
        body: true
    );
    post!(
        doc: "Invoke action degrees",
        name: degrees,
        path: "/functions/degrees",
        body: true
    );
    post!(
        doc: "Invoke action delta",
        name: delta,
        path: "/functions/delta",
        body: true
    );
    post!(
        doc: "Invoke action devSq",
        name: dev_sq,
        path: "/functions/devSq",
        body: true
    );
    post!(
        doc: "Invoke action dget",
        name: dget,
        path: "/functions/dget",
        body: true
    );
    post!(
        doc: "Invoke action disc",
        name: disc,
        path: "/functions/disc",
        body: true
    );
    post!(
        doc: "Invoke action dmax",
        name: dmax,
        path: "/functions/dmax",
        body: true
    );
    post!(
        doc: "Invoke action dmin",
        name: dmin,
        path: "/functions/dmin",
        body: true
    );
    post!(
        doc: "Invoke action dollar",
        name: dollar,
        path: "/functions/dollar",
        body: true
    );
    post!(
        doc: "Invoke action dollarDe",
        name: dollar_de,
        path: "/functions/dollarDe",
        body: true
    );
    post!(
        doc: "Invoke action dollarFr",
        name: dollar_fr,
        path: "/functions/dollarFr",
        body: true
    );
    post!(
        doc: "Invoke action dproduct",
        name: dproduct,
        path: "/functions/dproduct",
        body: true
    );
    post!(
        doc: "Invoke action dstDev",
        name: dst_dev,
        path: "/functions/dstDev",
        body: true
    );
    post!(
        doc: "Invoke action dstDevP",
        name: dst_dev_p,
        path: "/functions/dstDevP",
        body: true
    );
    post!(
        doc: "Invoke action dsum",
        name: dsum,
        path: "/functions/dsum",
        body: true
    );
    post!(
        doc: "Invoke action duration",
        name: duration,
        path: "/functions/duration",
        body: true
    );
    post!(
        doc: "Invoke action dvar",
        name: dvar,
        path: "/functions/dvar",
        body: true
    );
    post!(
        doc: "Invoke action dvarP",
        name: dvar_p,
        path: "/functions/dvarP",
        body: true
    );
    post!(
        doc: "Invoke action ecma_Ceiling",
        name: ecma_ceiling,
        path: "/functions/ecma_Ceiling",
        body: true
    );
    post!(
        doc: "Invoke action edate",
        name: edate,
        path: "/functions/edate",
        body: true
    );
    post!(
        doc: "Invoke action effect",
        name: effect,
        path: "/functions/effect",
        body: true
    );
    post!(
        doc: "Invoke action eoMonth",
        name: eo_month,
        path: "/functions/eoMonth",
        body: true
    );
    post!(
        doc: "Invoke action erf",
        name: erf,
        path: "/functions/erf",
        body: true
    );
    post!(
        doc: "Invoke action erfC",
        name: erf_c,
        path: "/functions/erfC",
        body: true
    );
    post!(
        doc: "Invoke action erfC_Precise",
        name: erf_c_precise,
        path: "/functions/erfC_Precise",
        body: true
    );
    post!(
        doc: "Invoke action erf_Precise",
        name: erf_precise,
        path: "/functions/erf_Precise",
        body: true
    );
    post!(
        doc: "Invoke action error_Type",
        name: error_type,
        path: "/functions/error_Type",
        body: true
    );
    post!(
        doc: "Invoke action even",
        name: even,
        path: "/functions/even",
        body: true
    );
    post!(
        doc: "Invoke action exact",
        name: exact,
        path: "/functions/exact",
        body: true
    );
    post!(
        doc: "Invoke action exp",
        name: exp,
        path: "/functions/exp",
        body: true
    );
    post!(
        doc: "Invoke action expon_Dist",
        name: expon_dist,
        path: "/functions/expon_Dist",
        body: true
    );
    post!(
        doc: "Invoke action f_Dist",
        name: f_dist,
        path: "/functions/f_Dist",
        body: true
    );
    post!(
        doc: "Invoke action f_Dist_RT",
        name: f_dist_rt,
        path: "/functions/f_Dist_RT",
        body: true
    );
    post!(
        doc: "Invoke action f_Inv",
        name: f_inv,
        path: "/functions/f_Inv",
        body: true
    );
    post!(
        doc: "Invoke action f_Inv_RT",
        name: f_inv_rt,
        path: "/functions/f_Inv_RT",
        body: true
    );
    post!(
        doc: "Invoke action fact",
        name: fact,
        path: "/functions/fact",
        body: true
    );
    post!(
        doc: "Invoke action factDouble",
        name: fact_double,
        path: "/functions/factDouble",
        body: true
    );
    post!(
        doc: "Invoke action false",
        name: _false,
        path: "/functions/false"
    );
    post!(
        doc: "Invoke action find",
        name: find,
        path: "/functions/find",
        body: true
    );
    post!(
        doc: "Invoke action findB",
        name: find_b,
        path: "/functions/findB",
        body: true
    );
    post!(
        doc: "Invoke action fisher",
        name: fisher,
        path: "/functions/fisher",
        body: true
    );
    post!(
        doc: "Invoke action fisherInv",
        name: fisher_inv,
        path: "/functions/fisherInv",
        body: true
    );
    post!(
        doc: "Invoke action fixed",
        name: fixed,
        path: "/functions/fixed",
        body: true
    );
    post!(
        doc: "Invoke action floor_Math",
        name: floor_math,
        path: "/functions/floor_Math",
        body: true
    );
    post!(
        doc: "Invoke action floor_Precise",
        name: floor_precise,
        path: "/functions/floor_Precise",
        body: true
    );
    post!(
        doc: "Invoke action fv",
        name: fv,
        path: "/functions/fv",
        body: true
    );
    post!(
        doc: "Invoke action fvschedule",
        name: fvschedule,
        path: "/functions/fvschedule",
        body: true
    );
    post!(
        doc: "Invoke action gamma",
        name: gamma,
        path: "/functions/gamma",
        body: true
    );
    post!(
        doc: "Invoke action gammaLn",
        name: gamma_ln,
        path: "/functions/gammaLn",
        body: true
    );
    post!(
        doc: "Invoke action gammaLn_Precise",
        name: gamma_ln_precise,
        path: "/functions/gammaLn_Precise",
        body: true
    );
    post!(
        doc: "Invoke action gamma_Dist",
        name: gamma_dist,
        path: "/functions/gamma_Dist",
        body: true
    );
    post!(
        doc: "Invoke action gamma_Inv",
        name: gamma_inv,
        path: "/functions/gamma_Inv",
        body: true
    );
    post!(
        doc: "Invoke action gauss",
        name: gauss,
        path: "/functions/gauss",
        body: true
    );
    post!(
        doc: "Invoke action gcd",
        name: gcd,
        path: "/functions/gcd",
        body: true
    );
    post!(
        doc: "Invoke action geStep",
        name: ge_step,
        path: "/functions/geStep",
        body: true
    );
    post!(
        doc: "Invoke action geoMean",
        name: geo_mean,
        path: "/functions/geoMean",
        body: true
    );
    post!(
        doc: "Invoke action harMean",
        name: har_mean,
        path: "/functions/harMean",
        body: true
    );
    post!(
        doc: "Invoke action hex2Bin",
        name: hex_2b_in,
        path: "/functions/hex2Bin",
        body: true
    );
    post!(
        doc: "Invoke action hex2Dec",
        name: hex_2_dec,
        path: "/functions/hex2Dec",
        body: true
    );
    post!(
        doc: "Invoke action hex2Oct",
        name: hex_20_ct,
        path: "/functions/hex2Oct",
        body: true
    );
    post!(
        doc: "Invoke action hlookup",
        name: hlookup,
        path: "/functions/hlookup",
        body: true
    );
    post!(
        doc: "Invoke action hour",
        name: hour,
        path: "/functions/hour",
        body: true
    );
    post!(
        doc: "Invoke action hypGeom_Dist",
        name: hyp_geom_dist,
        path: "/functions/hypGeom_Dist",
        body: true
    );
    post!(
        doc: "Invoke action hyperlink",
        name: hyperlink,
        path: "/functions/hyperlink",
        body: true
    );
    post!(
        doc: "Invoke action if",
        name: _if,
        path: "/functions/if",
        body: true
    );
    post!(
        doc: "Invoke action imAbs",
        name: im_abs,
        path: "/functions/imAbs",
        body: true
    );
    post!(
        doc: "Invoke action imArgument",
        name: im_argument,
        path: "/functions/imArgument",
        body: true
    );
    post!(
        doc: "Invoke action imConjugate",
        name: im_conjugate,
        path: "/functions/imConjugate",
        body: true
    );
    post!(
        doc: "Invoke action imCos",
        name: im_cos,
        path: "/functions/imCos",
        body: true
    );
    post!(
        doc: "Invoke action imCosh",
        name: im_cosh,
        path: "/functions/imCosh",
        body: true
    );
    post!(
        doc: "Invoke action imCot",
        name: im_cot,
        path: "/functions/imCot",
        body: true
    );
    post!(
        doc: "Invoke action imCsc",
        name: im_csc,
        path: "/functions/imCsc",
        body: true
    );
    post!(
        doc: "Invoke action imCsch",
        name: im_csch,
        path: "/functions/imCsch",
        body: true
    );
    post!(
        doc: "Invoke action imDiv",
        name: im_div,
        path: "/functions/imDiv",
        body: true
    );
    post!(
        doc: "Invoke action imExp",
        name: im_exp,
        path: "/functions/imExp",
        body: true
    );
    post!(
        doc: "Invoke action imLn",
        name: im_ln,
        path: "/functions/imLn",
        body: true
    );
    post!(
        doc: "Invoke action imLog10",
        name: im_log_10,
        path: "/functions/imLog10",
        body: true
    );
    post!(
        doc: "Invoke action imLog2",
        name: im_log_2,
        path: "/functions/imLog2",
        body: true
    );
    post!(
        doc: "Invoke action imPower",
        name: im_power,
        path: "/functions/imPower",
        body: true
    );
    post!(
        doc: "Invoke action imProduct",
        name: im_product,
        path: "/functions/imProduct",
        body: true
    );
    post!(
        doc: "Invoke action imReal",
        name: im_real,
        path: "/functions/imReal",
        body: true
    );
    post!(
        doc: "Invoke action imSec",
        name: im_sec,
        path: "/functions/imSec",
        body: true
    );
    post!(
        doc: "Invoke action imSech",
        name: im_sech,
        path: "/functions/imSech",
        body: true
    );
    post!(
        doc: "Invoke action imSin",
        name: im_sin,
        path: "/functions/imSin",
        body: true
    );
    post!(
        doc: "Invoke action imSinh",
        name: im_sinh,
        path: "/functions/imSinh",
        body: true
    );
    post!(
        doc: "Invoke action imSqrt",
        name: im_sqrt,
        path: "/functions/imSqrt",
        body: true
    );
    post!(
        doc: "Invoke action imSub",
        name: im_sub,
        path: "/functions/imSub",
        body: true
    );
    post!(
        doc: "Invoke action imSum",
        name: im_sum,
        path: "/functions/imSum",
        body: true
    );
    post!(
        doc: "Invoke action imTan",
        name: im_tan,
        path: "/functions/imTan",
        body: true
    );
    post!(
        doc: "Invoke action imaginary",
        name: imaginary,
        path: "/functions/imaginary",
        body: true
    );
    post!(
        doc: "Invoke action int",
        name: int,
        path: "/functions/int",
        body: true
    );
    post!(
        doc: "Invoke action intRate",
        name: int_rate,
        path: "/functions/intRate",
        body: true
    );
    post!(
        doc: "Invoke action ipmt",
        name: ipmt,
        path: "/functions/ipmt",
        body: true
    );
    post!(
        doc: "Invoke action irr",
        name: irr,
        path: "/functions/irr",
        body: true
    );
    post!(
        doc: "Invoke action isErr",
        name: is_err,
        path: "/functions/isErr",
        body: true
    );
    post!(
        doc: "Invoke action isError",
        name: is_error,
        path: "/functions/isError",
        body: true
    );
    post!(
        doc: "Invoke action isEven",
        name: is_even,
        path: "/functions/isEven",
        body: true
    );
    post!(
        doc: "Invoke action isFormula",
        name: is_formula,
        path: "/functions/isFormula",
        body: true
    );
    post!(
        doc: "Invoke action isLogical",
        name: is_logical,
        path: "/functions/isLogical",
        body: true
    );
    post!(
        doc: "Invoke action isNA",
        name: is_na,
        path: "/functions/isNA",
        body: true
    );
    post!(
        doc: "Invoke action isNonText",
        name: is_non_text,
        path: "/functions/isNonText",
        body: true
    );
    post!(
        doc: "Invoke action isNumber",
        name: is_number,
        path: "/functions/isNumber",
        body: true
    );
    post!(
        doc: "Invoke action isOdd",
        name: is_odd,
        path: "/functions/isOdd",
        body: true
    );
    post!(
        doc: "Invoke action isText",
        name: is_text,
        path: "/functions/isText",
        body: true
    );
    post!(
        doc: "Invoke action isoWeekNum",
        name: iso_week_num,
        path: "/functions/isoWeekNum",
        body: true
    );
    post!(
        doc: "Invoke action iso_Ceiling",
        name: iso_ceiling,
        path: "/functions/iso_Ceiling",
        body: true
    );
    post!(
        doc: "Invoke action ispmt",
        name: ispmt,
        path: "/functions/ispmt",
        body: true
    );
    post!(
        doc: "Invoke action isref",
        name: isref,
        path: "/functions/isref",
        body: true
    );
    post!(
        doc: "Invoke action kurt",
        name: kurt,
        path: "/functions/kurt",
        body: true
    );
    post!(
        doc: "Invoke action large",
        name: large,
        path: "/functions/large",
        body: true
    );
    post!(
        doc: "Invoke action lcm",
        name: lcm,
        path: "/functions/lcm",
        body: true
    );
    post!(
        doc: "Invoke action left",
        name: left,
        path: "/functions/left",
        body: true
    );
    post!(
        doc: "Invoke action leftb",
        name: leftb,
        path: "/functions/leftb",
        body: true
    );
    post!(
        doc: "Invoke action len",
        name: len,
        path: "/functions/len",
        body: true
    );
    post!(
        doc: "Invoke action lenb",
        name: lenb,
        path: "/functions/lenb",
        body: true
    );
    post!(
        doc: "Invoke action ln",
        name: ln,
        path: "/functions/ln",
        body: true
    );
    post!(
        doc: "Invoke action log",
        name: log,
        path: "/functions/log",
        body: true
    );
    post!(
        doc: "Invoke action log10",
        name: log_10,
        path: "/functions/log10",
        body: true
    );
    post!(
        doc: "Invoke action logNorm_Dist",
        name: log_norm_dist,
        path: "/functions/logNorm_Dist",
        body: true
    );
    post!(
        doc: "Invoke action logNorm_Inv",
        name: log_norm_inv,
        path: "/functions/logNorm_Inv",
        body: true
    );
    post!(
        doc: "Invoke action lookup",
        name: lookup,
        path: "/functions/lookup",
        body: true
    );
    post!(
        doc: "Invoke action lower",
        name: lower,
        path: "/functions/lower",
        body: true
    );
    post!(
        doc: "Invoke action match",
        name: _match,
        path: "/functions/match",
        body: true
    );
    post!(
        doc: "Invoke action max",
        name: max,
        path: "/functions/max",
        body: true
    );
    post!(
        doc: "Invoke action maxA",
        name: max_a,
        path: "/functions/maxA",
        body: true
    );
    post!(
        doc: "Invoke action mduration",
        name: mduration,
        path: "/functions/mduration",
        body: true
    );
    post!(
        doc: "Invoke action median",
        name: median,
        path: "/functions/median",
        body: true
    );
    post!(
        doc: "Invoke action mid",
        name: mid,
        path: "/functions/mid",
        body: true
    );
    post!(
        doc: "Invoke action midb",
        name: midb,
        path: "/functions/midb",
        body: true
    );
    post!(
        doc: "Invoke action min",
        name: min,
        path: "/functions/min",
        body: true
    );
    post!(
        doc: "Invoke action minA",
        name: min_a,
        path: "/functions/minA",
        body: true
    );
    post!(
        doc: "Invoke action minute",
        name: minute,
        path: "/functions/minute",
        body: true
    );
    post!(
        doc: "Invoke action mirr",
        name: mirr,
        path: "/functions/mirr",
        body: true
    );
    post!(
        doc: "Invoke action mod",
        name: _mod,
        path: "/functions/mod",
        body: true
    );
    post!(
        doc: "Invoke action month",
        name: month,
        path: "/functions/month",
        body: true
    );
    post!(
        doc: "Invoke action mround",
        name: mround,
        path: "/functions/mround",
        body: true
    );
    post!(
        doc: "Invoke action multiNomial",
        name: multi_nomial,
        path: "/functions/multiNomial",
        body: true
    );
    post!(
        doc: "Invoke action n",
        name: n,
        path: "/functions/n",
        body: true
    );
    post!(
        doc: "Invoke action na",
        name: na,
        path: "/functions/na"
    );
    post!(
        doc: "Invoke action negBinom_Dist",
        name: neg_binom_dist,
        path: "/functions/negBinom_Dist",
        body: true
    );
    post!(
        doc: "Invoke action networkDays",
        name: network_days,
        path: "/functions/networkDays",
        body: true
    );
    post!(
        doc: "Invoke action networkDays_Intl",
        name: network_days_intl,
        path: "/functions/networkDays_Intl",
        body: true
    );
    post!(
        doc: "Invoke action nominal",
        name: nominal,
        path: "/functions/nominal",
        body: true
    );
    post!(
        doc: "Invoke action norm_Dist",
        name: norm_dist,
        path: "/functions/norm_Dist",
        body: true
    );
    post!(
        doc: "Invoke action norm_Inv",
        name: norm_inv,
        path: "/functions/norm_Inv",
        body: true
    );
    post!(
        doc: "Invoke action norm_S_Dist",
        name: norm_s_dist,
        path: "/functions/norm_S_Dist",
        body: true
    );
    post!(
        doc: "Invoke action norm_S_Inv",
        name: norm_s_inv,
        path: "/functions/norm_S_Inv",
        body: true
    );
    post!(
        doc: "Invoke action not",
        name: not,
        path: "/functions/not",
        body: true
    );
    post!(
        doc: "Invoke action now",
        name: now,
        path: "/functions/now"
    );
    post!(
        doc: "Invoke action nper",
        name: nper,
        path: "/functions/nper",
        body: true
    );
    post!(
        doc: "Invoke action npv",
        name: npv,
        path: "/functions/npv",
        body: true
    );
    post!(
        doc: "Invoke action numberValue",
        name: number_value,
        path: "/functions/numberValue",
        body: true
    );
    post!(
        doc: "Invoke action oct2Bin",
        name: oct_2_bin,
        path: "/functions/oct2Bin",
        body: true
    );
    post!(
        doc: "Invoke action oct2Dec",
        name: oct_2_dec,
        path: "/functions/oct2Dec",
        body: true
    );
    post!(
        doc: "Invoke action oct2Hex",
        name: oct_2_hex,
        path: "/functions/oct2Hex",
        body: true
    );
    post!(
        doc: "Invoke action odd",
        name: odd,
        path: "/functions/odd",
        body: true
    );
    post!(
        doc: "Invoke action oddFPrice",
        name: odd_f_price,
        path: "/functions/oddFPrice",
        body: true
    );
    post!(
        doc: "Invoke action oddFYield",
        name: odd_f_yield,
        path: "/functions/oddFYield",
        body: true
    );
    post!(
        doc: "Invoke action oddLPrice",
        name: odd_l_price,
        path: "/functions/oddLPrice",
        body: true
    );
    post!(
        doc: "Invoke action oddLYield",
        name: odd_l_yield,
        path: "/functions/oddLYield",
        body: true
    );
    post!(
        doc: "Invoke action or",
        name: or,
        path: "/functions/or",
        body: true
    );
    post!(
        doc: "Invoke action pduration",
        name: pduration,
        path: "/functions/pduration",
        body: true
    );
    post!(
        doc: "Invoke action percentRank_Exc",
        name: percent_rank_exc,
        path: "/functions/percentRank_Exc",
        body: true
    );
    post!(
        doc: "Invoke action percentRank_Inc",
        name: percent_rank_inc,
        path: "/functions/percentRank_Inc",
        body: true
    );
    post!(
        doc: "Invoke action percentile_Exc",
        name: percentile_exc,
        path: "/functions/percentile_Exc",
        body: true
    );
    post!(
        doc: "Invoke action percentile_Inc",
        name: percentile_inc,
        path: "/functions/percentile_Inc",
        body: true
    );
    post!(
        doc: "Invoke action permut",
        name: permut,
        path: "/functions/permut",
        body: true
    );
    post!(
        doc: "Invoke action permutationa",
        name: permutationa,
        path: "/functions/permutationa",
        body: true
    );
    post!(
        doc: "Invoke action phi",
        name: phi,
        path: "/functions/phi",
        body: true
    );
    post!(
        doc: "Invoke action pi",
        name: pi,
        path: "/functions/pi"
    );
    post!(
        doc: "Invoke action pmt",
        name: pmt,
        path: "/functions/pmt",
        body: true
    );
    post!(
        doc: "Invoke action poisson_Dist",
        name: poisson_dist,
        path: "/functions/poisson_Dist",
        body: true
    );
    post!(
        doc: "Invoke action power",
        name: power,
        path: "/functions/power",
        body: true
    );
    post!(
        doc: "Invoke action ppmt",
        name: ppmt,
        path: "/functions/ppmt",
        body: true
    );
    post!(
        doc: "Invoke action price",
        name: price,
        path: "/functions/price",
        body: true
    );
    post!(
        doc: "Invoke action priceDisc",
        name: price_disc,
        path: "/functions/priceDisc",
        body: true
    );
    post!(
        doc: "Invoke action priceMat",
        name: price_mat,
        path: "/functions/priceMat",
        body: true
    );
    post!(
        doc: "Invoke action product",
        name: product,
        path: "/functions/product",
        body: true
    );
    post!(
        doc: "Invoke action proper",
        name: proper,
        path: "/functions/proper",
        body: true
    );
    post!(
        doc: "Invoke action pv",
        name: pv,
        path: "/functions/pv",
        body: true
    );
    post!(
        doc: "Invoke action quartile_Exc",
        name: quartile_exc,
        path: "/functions/quartile_Exc",
        body: true
    );
    post!(
        doc: "Invoke action quartile_Inc",
        name: quartile_inc,
        path: "/functions/quartile_Inc",
        body: true
    );
    post!(
        doc: "Invoke action quotient",
        name: quotient,
        path: "/functions/quotient",
        body: true
    );
    post!(
        doc: "Invoke action radians",
        name: radians,
        path: "/functions/radians",
        body: true
    );
    post!(
        doc: "Invoke action rand",
        name: rand,
        path: "/functions/rand"
    );
    post!(
        doc: "Invoke action randBetween",
        name: rand_between,
        path: "/functions/randBetween",
        body: true
    );
    post!(
        doc: "Invoke action rank_Avg",
        name: rank_avg,
        path: "/functions/rank_Avg",
        body: true
    );
    post!(
        doc: "Invoke action rank_Eq",
        name: rank_eq,
        path: "/functions/rank_Eq",
        body: true
    );
    post!(
        doc: "Invoke action rate",
        name: rate,
        path: "/functions/rate",
        body: true
    );
    post!(
        doc: "Invoke action received",
        name: received,
        path: "/functions/received",
        body: true
    );
    post!(
        doc: "Invoke action replace",
        name: replace,
        path: "/functions/replace",
        body: true
    );
    post!(
        doc: "Invoke action replaceB",
        name: replace_b,
        path: "/functions/replaceB",
        body: true
    );
    post!(
        doc: "Invoke action rept",
        name: rept,
        path: "/functions/rept",
        body: true
    );
    post!(
        doc: "Invoke action right",
        name: right,
        path: "/functions/right",
        body: true
    );
    post!(
        doc: "Invoke action rightb",
        name: rightb,
        path: "/functions/rightb",
        body: true
    );
    post!(
        doc: "Invoke action roman",
        name: roman,
        path: "/functions/roman",
        body: true
    );
    post!(
        doc: "Invoke action round",
        name: round,
        path: "/functions/round",
        body: true
    );
    post!(
        doc: "Invoke action roundDown",
        name: round_down,
        path: "/functions/roundDown",
        body: true
    );
    post!(
        doc: "Invoke action roundUp",
        name: round_up,
        path: "/functions/roundUp",
        body: true
    );
    post!(
        doc: "Invoke action rows",
        name: rows,
        path: "/functions/rows",
        body: true
    );
    post!(
        doc: "Invoke action rri",
        name: rri,
        path: "/functions/rri",
        body: true
    );
    post!(
        doc: "Invoke action sec",
        name: sec,
        path: "/functions/sec",
        body: true
    );
    post!(
        doc: "Invoke action sech",
        name: sech,
        path: "/functions/sech",
        body: true
    );
    post!(
        doc: "Invoke action second",
        name: second,
        path: "/functions/second",
        body: true
    );
    post!(
        doc: "Invoke action seriesSum",
        name: series_sum,
        path: "/functions/seriesSum",
        body: true
    );
    post!(
        doc: "Invoke action sheet",
        name: sheet,
        path: "/functions/sheet",
        body: true
    );
    post!(
        doc: "Invoke action sheets",
        name: sheets,
        path: "/functions/sheets",
        body: true
    );
    post!(
        doc: "Invoke action sign",
        name: sign,
        path: "/functions/sign",
        body: true
    );
    post!(
        doc: "Invoke action sin",
        name: sin,
        path: "/functions/sin",
        body: true
    );
    post!(
        doc: "Invoke action sinh",
        name: sinh,
        path: "/functions/sinh",
        body: true
    );
    post!(
        doc: "Invoke action skew",
        name: skew,
        path: "/functions/skew",
        body: true
    );
    post!(
        doc: "Invoke action skew_p",
        name: skew_p,
        path: "/functions/skew_p",
        body: true
    );
    post!(
        doc: "Invoke action sln",
        name: sln,
        path: "/functions/sln",
        body: true
    );
    post!(
        doc: "Invoke action small",
        name: small,
        path: "/functions/small",
        body: true
    );
    post!(
        doc: "Invoke action sqrt",
        name: sqrt,
        path: "/functions/sqrt",
        body: true
    );
    post!(
        doc: "Invoke action sqrtPi",
        name: sqrt_pi,
        path: "/functions/sqrtPi",
        body: true
    );
    post!(
        doc: "Invoke action stDevA",
        name: st_dev_a,
        path: "/functions/stDevA",
        body: true
    );
    post!(
        doc: "Invoke action stDevPA",
        name: st_dev_pa,
        path: "/functions/stDevPA",
        body: true
    );
    post!(
        doc: "Invoke action stDev_P",
        name: st_dev_p,
        path: "/functions/stDev_P",
        body: true
    );
    post!(
        doc: "Invoke action stDev_S",
        name: st_dev_s,
        path: "/functions/stDev_S",
        body: true
    );
    post!(
        doc: "Invoke action standardize",
        name: standardize,
        path: "/functions/standardize",
        body: true
    );
    post!(
        doc: "Invoke action substitute",
        name: substitute,
        path: "/functions/substitute",
        body: true
    );
    post!(
        doc: "Invoke action subtotal",
        name: subtotal,
        path: "/functions/subtotal",
        body: true
    );
    post!(
        doc: "Invoke action sum",
        name: sum,
        path: "/functions/sum",
        body: true
    );
    post!(
        doc: "Invoke action sumIf",
        name: sum_if,
        path: "/functions/sumIf",
        body: true
    );
    post!(
        doc: "Invoke action sumIfs",
        name: sum_ifs,
        path: "/functions/sumIfs",
        body: true
    );
    post!(
        doc: "Invoke action sumSq",
        name: sum_sq,
        path: "/functions/sumSq",
        body: true
    );
    post!(
        doc: "Invoke action syd",
        name: syd,
        path: "/functions/syd",
        body: true
    );
    post!(
        doc: "Invoke action t",
        name: t,
        path: "/functions/t",
        body: true
    );
    post!(
        doc: "Invoke action t_Dist",
        name: t_dist,
        path: "/functions/t_Dist",
        body: true
    );
    post!(
        doc: "Invoke action t_Dist_2T",
        name: t_dist_2t,
        path: "/functions/t_Dist_2T",
        body: true
    );
    post!(
        doc: "Invoke action t_Dist_RT",
        name: t_dist_rt,
        path: "/functions/t_Dist_RT",
        body: true
    );
    post!(
        doc: "Invoke action t_Inv",
        name: t_inv,
        path: "/functions/t_Inv",
        body: true
    );
    post!(
        doc: "Invoke action t_Inv_2T",
        name: t_inv_2t,
        path: "/functions/t_Inv_2T",
        body: true
    );
    post!(
        doc: "Invoke action tan",
        name: tan,
        path: "/functions/tan",
        body: true
    );
    post!(
        doc: "Invoke action tanh",
        name: tanh,
        path: "/functions/tanh",
        body: true
    );
    post!(
        doc: "Invoke action tbillEq",
        name: tbill_eq,
        path: "/functions/tbillEq",
        body: true
    );
    post!(
        doc: "Invoke action tbillPrice",
        name: tbill_price,
        path: "/functions/tbillPrice",
        body: true
    );
    post!(
        doc: "Invoke action tbillYield",
        name: tbill_yield,
        path: "/functions/tbillYield",
        body: true
    );
    post!(
        doc: "Invoke action text",
        name: text,
        path: "/functions/text",
        body: true
    );
    post!(
        doc: "Invoke action time",
        name: time,
        path: "/functions/time",
        body: true
    );
    post!(
        doc: "Invoke action timevalue",
        name: timevalue,
        path: "/functions/timevalue",
        body: true
    );
    post!(
        doc: "Invoke action today",
        name: today,
        path: "/functions/today"
    );
    post!(
        doc: "Invoke action trim",
        name: trim,
        path: "/functions/trim",
        body: true
    );
    post!(
        doc: "Invoke action trimMean",
        name: trim_mean,
        path: "/functions/trimMean",
        body: true
    );
    post!(
        doc: "Invoke action true",
        name: _true,
        path: "/functions/true"
    );
    post!(
        doc: "Invoke action trunc",
        name: trunc,
        path: "/functions/trunc",
        body: true
    );
    post!(
        doc: "Invoke action type",
        name: _type,
        path: "/functions/type",
        body: true
    );
    post!(
        doc: "Invoke action unichar",
        name: unichar,
        path: "/functions/unichar",
        body: true
    );
    post!(
        doc: "Invoke action unicode",
        name: unicode,
        path: "/functions/unicode",
        body: true
    );
    post!(
        doc: "Invoke action upper",
        name: upper,
        path: "/functions/upper",
        body: true
    );
    post!(
        doc: "Invoke action usdollar",
        name: usdollar,
        path: "/functions/usdollar",
        body: true
    );
    post!(
        doc: "Invoke action value",
        name: value,
        path: "/functions/value",
        body: true
    );
    post!(
        doc: "Invoke action varA",
        name: var_a,
        path: "/functions/varA",
        body: true
    );
    post!(
        doc: "Invoke action varPA",
        name: var_pa,
        path: "/functions/varPA",
        body: true
    );
    post!(
        doc: "Invoke action var_P",
        name: var_p,
        path: "/functions/var_P",
        body: true
    );
    post!(
        doc: "Invoke action var_S",
        name: var_s,
        path: "/functions/var_S",
        body: true
    );
    post!(
        doc: "Invoke action vdb",
        name: vdb,
        path: "/functions/vdb",
        body: true
    );
    post!(
        doc: "Invoke action vlookup",
        name: vlookup,
        path: "/functions/vlookup",
        body: true
    );
    post!(
        doc: "Invoke action weekNum",
        name: week_num,
        path: "/functions/weekNum",
        body: true
    );
    post!(
        doc: "Invoke action weekday",
        name: weekday,
        path: "/functions/weekday",
        body: true
    );
    post!(
        doc: "Invoke action weibull_Dist",
        name: weibull_dist,
        path: "/functions/weibull_Dist",
        body: true
    );
    post!(
        doc: "Invoke action workDay",
        name: work_day,
        path: "/functions/workDay",
        body: true
    );
    post!(
        doc: "Invoke action workDay_Intl",
        name: work_day_intl,
        path: "/functions/workDay_Intl",
        body: true
    );
    post!(
        doc: "Invoke action xirr",
        name: xirr,
        path: "/functions/xirr",
        body: true
    );
    post!(
        doc: "Invoke action xnpv",
        name: xnpv,
        path: "/functions/xnpv",
        body: true
    );
    post!(
        doc: "Invoke action xor",
        name: xor,
        path: "/functions/xor",
        body: true
    );
    post!(
        doc: "Invoke action year",
        name: year,
        path: "/functions/year",
        body: true
    );
    post!(
        doc: "Invoke action yearFrac",
        name: year_frac,
        path: "/functions/yearFrac",
        body: true
    );
    post!(
        doc: "Invoke action yield",
        name: _yield,
        path: "/functions/yield",
        body: true
    );
    post!(
        doc: "Invoke action yieldDisc",
        name: yield_disc,
        path: "/functions/yieldDisc",
        body: true
    );
    post!(
        doc: "Invoke action yieldMat",
        name: yield_mat,
        path: "/functions/yieldMat",
        body: true
    );
    post!(
        doc: "Invoke action z_Test",
        name: z_test,
        path: "/functions/z_Test",
        body: true
    );
}
