#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneCovariate {
    pub row_name: char,
    pub cov1: u8,
    pub cov2: u8,
    pub cov3: u8,
}

pub const GENE_COV: [GeneCovariate; 26] = [
    GeneCovariate {
        row_name: 'A',
        cov1: 1,
        cov2: 1,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'B',
        cov1: 1,
        cov2: 1,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'C',
        cov1: 1,
        cov2: 1,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'D',
        cov1: 1,
        cov2: 1,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'E',
        cov1: 1,
        cov2: 2,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'F',
        cov1: 1,
        cov2: 2,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'G',
        cov1: 1,
        cov2: 2,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'H',
        cov1: 1,
        cov2: 2,
        cov3: 1,
    },
    GeneCovariate {
        row_name: 'I',
        cov1: 1,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'J',
        cov1: 1,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'K',
        cov1: 1,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'L',
        cov1: 1,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'M',
        cov1: 1,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'N',
        cov1: 2,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'O',
        cov1: 2,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'P',
        cov1: 2,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'Q',
        cov1: 2,
        cov2: 2,
        cov3: 2,
    },
    GeneCovariate {
        row_name: 'R',
        cov1: 2,
        cov2: 2,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'S',
        cov1: 2,
        cov2: 2,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'T',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'U',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'V',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'W',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'X',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'Y',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
    GeneCovariate {
        row_name: 'Z',
        cov1: 2,
        cov2: 1,
        cov3: 3,
    },
];

pub fn gene_covariate(row_name: char) -> Option<&'static GeneCovariate> {
    GENE_COV.iter().find(|row| row.row_name == row_name)
}
