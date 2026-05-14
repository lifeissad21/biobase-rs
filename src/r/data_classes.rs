use std::collections::BTreeMap;

use super::versioned_classes::Versioned;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Number(f64),
    Text(String),
    Vector(Vec<Value>),
}

impl Value {
    pub fn is_missing(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Number(value) => value.is_nan(),
            Value::Vector(values) => values.iter().any(Value::is_missing),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    values: Vec<T>,
    rows: usize,
    columns: usize,
    row_names: Option<Vec<String>>,
    column_names: Option<Vec<String>>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, columns: usize, values: Vec<T>) -> Result<Self, String> {
        if values.len() != rows * columns {
            return Err(format!(
                "matrix dimensions require {} values, got {}",
                rows * columns,
                values.len()
            ));
        }
        Ok(Self {
            values,
            rows,
            columns,
            row_names: None,
            column_names: None,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [T] {
        &mut self.values
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.values.get(column * self.rows + row)
    }

    pub fn set_dimnames(
        &mut self,
        row_names: Option<Vec<String>>,
        column_names: Option<Vec<String>>,
    ) -> Result<(), String> {
        if let Some(names) = &row_names {
            if names.len() != self.rows {
                return Err("row name count does not match row count".to_string());
            }
        }
        if let Some(names) = &column_names {
            if names.len() != self.columns {
                return Err("column name count does not match column count".to_string());
            }
        }
        self.row_names = row_names;
        self.column_names = column_names;
        Ok(())
    }

    pub fn row_names(&self) -> Option<&[String]> {
        self.row_names.as_deref()
    }

    pub fn column_names(&self) -> Option<&[String]> {
        self.column_names.as_deref()
    }
}

pub type DataFrame = BTreeMap<String, Vec<Value>>;
pub type AssayData = BTreeMap<String, Matrix<f64>>;

#[derive(Debug, Clone)]
pub struct Aggregator<T, Init, Agg>
where
    Init: Fn(&str, &T) -> T,
    Agg: Fn(&str, &T, &T) -> T,
{
    values: BTreeMap<String, T>,
    initfun: Init,
    aggfun: Agg,
}

impl<T, Init, Agg> Aggregator<T, Init, Agg>
where
    T: Clone,
    Init: Fn(&str, &T) -> T,
    Agg: Fn(&str, &T, &T) -> T,
{
    pub fn new(initfun: Init, aggfun: Agg) -> Self {
        Self {
            values: BTreeMap::new(),
            initfun,
            aggfun,
        }
    }

    pub fn aggregate(&mut self, name: &str, value: T) {
        let next = match self.values.get(name) {
            Some(current) => (self.aggfun)(name, current, &value),
            None => (self.initfun)(name, &value),
        };
        self.values.insert(name.to_string(), next);
    }

    pub fn values(&self) -> &BTreeMap<String, T> {
        &self.values
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container<T> {
    values: Vec<T>,
    content: String,
    locked: bool,
}

impl<T> Container<T> {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            values: Vec::new(),
            content: content.into(),
            locked: false,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), String> {
        if self.locked {
            return Err("container is locked".to_string());
        }
        self.values.push(value);
        Ok(())
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miaxe {
    versioned: Versioned,
}

impl Miaxe {
    pub fn new(versioned: Versioned) -> Self {
        Self { versioned }
    }

    pub fn versioned(&self) -> &Versioned {
        &self.versioned
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Miame {
    pub name: String,
    pub lab: String,
    pub contact: String,
    pub title: String,
    pub abstract_text: String,
    pub url: String,
    pub pub_med_ids: Vec<String>,
    pub samples: Vec<Value>,
    pub hybridizations: Vec<Value>,
    pub norm_controls: Vec<Value>,
    pub preprocessing: Vec<Value>,
    pub other: Vec<Value>,
}

impl Default for Miame {
    fn default() -> Self {
        Self {
            name: String::new(),
            lab: String::new(),
            contact: String::new(),
            title: String::new(),
            abstract_text: String::new(),
            url: String::new(),
            pub_med_ids: Vec::new(),
            samples: Vec::new(),
            hybridizations: Vec::new(),
            norm_controls: Vec::new(),
            preprocessing: Vec::new(),
            other: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedDataFrame {
    data: DataFrame,
    var_metadata: DataFrame,
    dim_labels: [String; 2],
}

impl AnnotatedDataFrame {
    pub fn new(data: DataFrame, var_metadata: DataFrame, dim_labels: [String; 2]) -> Self {
        Self {
            data,
            var_metadata,
            dim_labels,
        }
    }

    pub fn empty(row_label: &str, column_label: &str) -> Self {
        Self::new(
            DataFrame::new(),
            DataFrame::new(),
            [row_label.to_string(), column_label.to_string()],
        )
    }

    pub fn data(&self) -> &DataFrame {
        &self.data
    }

    pub fn var_metadata(&self) -> &DataFrame {
        &self.var_metadata
    }

    pub fn dim_labels(&self) -> [&str; 2] {
        [&self.dim_labels[0], &self.dim_labels[1]]
    }

    pub fn var_labels(&self) -> Vec<&str> {
        self.data.keys().map(String::as_str).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ESet {
    pub assay_data: AssayData,
    pub pheno_data: AnnotatedDataFrame,
    pub feature_data: AnnotatedDataFrame,
    pub experiment_data: Miame,
    pub annotation: String,
    pub protocol_data: AnnotatedDataFrame,
}

impl ESet {
    pub fn new(assay_data: AssayData) -> Self {
        Self {
            assay_data,
            pheno_data: AnnotatedDataFrame::empty("sampleNames", "sampleColumns"),
            feature_data: AnnotatedDataFrame::empty("featureNames", "featureColumns"),
            experiment_data: Miame::default(),
            annotation: String::new(),
            protocol_data: AnnotatedDataFrame::empty("sampleNames", "sampleColumns"),
        }
    }

    pub fn assay_data_element(&self, name: &str) -> Option<&Matrix<f64>> {
        self.assay_data.get(name)
    }

    pub fn assay_data_element_mut(&mut self, name: &str) -> Option<&mut Matrix<f64>> {
        self.assay_data.get_mut(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionSet {
    pub base: ESet,
}

impl ExpressionSet {
    pub fn from_exprs(exprs: Matrix<f64>) -> Self {
        let mut assay_data = AssayData::new();
        assay_data.insert("exprs".to_string(), exprs);
        Self {
            base: ESet::new(assay_data),
        }
    }

    pub fn exprs(&self) -> Option<&Matrix<f64>> {
        self.base.assay_data_element("exprs")
    }

    pub fn exprs_mut(&mut self) -> Option<&mut Matrix<f64>> {
        self.base.assay_data_element_mut("exprs")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NChannelSet {
    pub base: ESet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSet {
    pub base: ESet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnpSet {
    pub base: ESet,
}

impl SnpSet {
    pub fn snp_call(&self) -> Option<&Matrix<f64>> {
        self.base.assay_data_element("call")
    }

    pub fn snp_call_probability(&self) -> Option<&Matrix<f64>> {
        self.base.assay_data_element("callProbability")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarObject {
    Logical(Option<bool>),
    Character(String),
    Integer(Option<i64>),
    Numeric(Option<f64>),
}
