use polars::prelude::*;
use polars_core::prelude::*;
use ndarray::prelude::*;

/// Convert df to ndarray, and back
pub(super) fn unit(df: DataFrame) -> PolarsResult<DataFrame> {
    println!("---Rust world---");
    // A DataFrame is built upon a Vec<Series> where the Series have the same length.
    println!("DF: {:?}", df);

    let ndarray = df_to_ndarray::<Float64Type>(df).unwrap();
    println!("NDARRAY: {:?}", ndarray);
    // TODO: Convert ndarray back to df, but for now just return the df
    let res = ndarray_to_df::<Float64Type>(ndarray).unwrap();
    println!("RES DF: {:?}", res);
    Ok(res)
}

/// Placeholder for 2D df transformations
fn df_to_ndarray<N>(df: DataFrame) -> PolarsResult<Array2<N::Native>> where
        N: PolarsNumericType {
    let arr = df.to_ndarray::<N>().unwrap();
    Ok(arr)
}

fn ndarray_to_df<N>(_ndarray: Array2<N::Native>) -> PolarsResult<DataFrame> where
        N: PolarsNumericType {
    let df = df!("TEST" => &["a", "b", "c"]).unwrap();
    Ok(df)
}