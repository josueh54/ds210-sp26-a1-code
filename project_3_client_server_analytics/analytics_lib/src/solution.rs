use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

//helper function
fn helper_function(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(column_name, expected_value) => {
            let col_index = dataset.column_index(column_name);
            let actual_value = row.get_value(col_index);
            actual_value == expected_value
        }

        Condition::Not(inner) => {
            !helper_function(row, dataset, inner)
        }

        Condition::And(left, right) => {
            helper_function(row, dataset, left)
                && helper_function(row, dataset, right)
        }

        Condition::Or(left, right) => {
            helper_function(row, dataset, left)
                || helper_function(row, dataset, right)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset 
{
    let mut filtered_dataset = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() 
    {
        if helper_function(row, dataset, filter) 
        {
            filtered_dataset.add_row(row.clone());
        }
    }
    return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> 
{
    let group_by_column_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    let mut grouped_datasets: HashMap<Value, Dataset> = HashMap::new();

    for row in dataset.into_iter() 
    {
        let group_value = row.get_value(group_by_column_index).clone();
        
        match grouped_datasets.get_mut(&group_value) 
        {
            Some(grouped_dataset) => 
            {
                grouped_dataset.add_row(row);
            },
            None => 
            {
                let mut new_grouped_dataset = Dataset::new(columns.clone());
                new_grouped_dataset.add_row(row);
                grouped_datasets.insert(group_value, new_grouped_dataset);
            }
        }
    }

    return grouped_datasets;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut results: HashMap<Value, Value> = HashMap::new();
    for (group_value, group_dataset) in dataset {
        match aggregation {
            Aggregation::Count(column_name) => {
                let count = group_dataset.len() as i32;
                results.insert(group_value, Value::Integer(count));
            }

            Aggregation::Sum(column_name) => {
                let col_index = group_dataset.column_index(column_name);
                let mut sum = 0;

                for row in group_dataset.iter() {
                    if let Value::Integer(v) = row.get_value(col_index) {
                        sum += *v;
                    }
                }

                results.insert(group_value, Value::Integer(sum));
            }

            Aggregation::Average(column_name) => {
                let col_index = group_dataset.column_index(column_name);
                let mut sum = 0;
                let mut count = 0;

                for row in group_dataset.iter() {
                    if let Value::Integer(v) = row.get_value(col_index) {
                        sum += *v;
                        count += 1;
                    }
                }

                let avg = if count > 0 { sum / count } else { 0 };
                results.insert(group_value, Value::Integer(avg));
            }
        }
    }
    results
}


pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}