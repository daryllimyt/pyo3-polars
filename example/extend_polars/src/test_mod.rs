use ndarray::{ArrayBase, Dimension, OwnedRepr, Ix2};
use polars::{
    export::{
        arrow::types::PrimitiveType,
        num::{FromPrimitive, ToPrimitive},
    },
    prelude::*,
};
// use polars_core::prelude::*;

/// Convert df to ndarray, and back
pub(super) fn unit(df: DataFrame) -> PolarsResult<DataFrame> {
    println!("---Rust world---");
    // A DataFrame is built upon a Vec<Series> where the Series have the same length.
    println!("DF: {:?}", df);

    // let ndarray = df_to_ndarray::<Float64Type>(df).unwrap();
    let ndarray = df.to_ndarray::<Float64Type>().unwrap();
    println!("NDARRAY: {:?}", ndarray);

    // TODO: Convert ndarray back to df, but for now just return the df
    let res = ndarray_to_df::<f64, Ix2>(&ndarray, vec!["a", "b"]).unwrap();
    println!("RES DF: {:?}", res);
    Ok(res)
}

// /// Temporary wrapper: Don't know if we can make this more generic, i.e. to support >2D arrays?
// fn df_to_ndarray<N>(df: DataFrame) -> PolarsResult<Array2<N::Native>>
// where
//     N: PolarsNumericType,
// {
//     let arr = df.to_ndarray::<N>().unwrap();
//     Ok(arr)
// }

/// from mitril-security bastionlab 
/// Don't think this is zc but we can change this later
pub fn ndarray_to_df<T, D: Dimension>(
    arr: &ArrayBase<OwnedRepr<T>, D>,
    col_names: Vec<&str>,
) -> PolarsResult<DataFrame>
where
    T: NumericNative + FromPrimitive + ToPrimitive,
{
    let mut lanes: Vec<Series> = vec![]; // This is just returning a new object

    for (i, col) in arr.columns().into_iter().enumerate() {
        match col.as_slice() {
            Some(d) => {
                if let PrimitiveType::Float64 = T::PRIMITIVE {
                    let d = d
                        .into_iter()
                        .map(|v| v.to_f64().unwrap())
                        .collect::<Vec<_>>();
                    let series = Series::from(Float64Chunked::new_vec(col_names[i], d));
                    lanes.push(series);
                }
                if let PrimitiveType::Float32 = T::PRIMITIVE {
                    let d = d
                        .into_iter()
                        .map(|v| v.to_f32().unwrap())
                        .collect::<Vec<_>>();
                    let series = Series::from(Float32Chunked::new_vec(col_names[i], d));
                    lanes.push(series);
                }
                if let PrimitiveType::UInt64 = T::PRIMITIVE {
                    let d = d
                        .into_iter()
                        .map(|v| v.to_u64().unwrap())
                        .collect::<Vec<_>>();
                    let series = Series::from(UInt64Chunked::new_vec(col_names[i], d));
                    lanes.push(series);
                }
                if let PrimitiveType::UInt32 = T::PRIMITIVE {
                    let d = d
                        .into_iter()
                        .map(|v| v.to_u32().unwrap())
                        .collect::<Vec<_>>();
                    let series = Series::from(UInt32Chunked::new_vec("col", d));
                    lanes.push(series);
                }
                // This could be expanded... for now, only (f64,f32, u64, and u32) are supported.
            }
            None => {
                return Err(PolarsError::NoData(polars::error::ErrString::Borrowed(
                    "Could not convert column to slice",
                )));
            }
        }
    }

    DataFrame::new(lanes)
}
