#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideChainProperty {
    Nonpolar,
    Polar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AminoAcid {
    pub name: &'static str,
    pub one_letter: char,
    pub three_letter: &'static str,
    pub side_chain_property: SideChainProperty,
    pub hydrophilic: Option<bool>,
    pub acidic: Option<bool>,
}

pub const AA_MAP: [AminoAcid; 20] = [
    AminoAcid {
        name: "alanine",
        one_letter: 'A',
        three_letter: "ala",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "cysteine",
        one_letter: 'C',
        three_letter: "cys",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "aspartic.acid",
        one_letter: 'D',
        three_letter: "asp",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(true),
    },
    AminoAcid {
        name: "glutamic.acid",
        one_letter: 'E',
        three_letter: "glu",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(true),
    },
    AminoAcid {
        name: "phenylalanine",
        one_letter: 'F',
        three_letter: "phe",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "glycine",
        one_letter: 'G',
        three_letter: "gly",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "histidine",
        one_letter: 'H',
        three_letter: "his",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(false),
    },
    AminoAcid {
        name: "isoleucine",
        one_letter: 'I',
        three_letter: "ile",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "lysine",
        one_letter: 'K',
        three_letter: "lys",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(false),
    },
    AminoAcid {
        name: "leucine",
        one_letter: 'L',
        three_letter: "leu",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "methionine",
        one_letter: 'M',
        three_letter: "met",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "asparagine",
        one_letter: 'N',
        three_letter: "asn",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(false),
    },
    AminoAcid {
        name: "proline",
        one_letter: 'P',
        three_letter: "pro",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "glutamine",
        one_letter: 'Q',
        three_letter: "gln",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(false),
    },
    AminoAcid {
        name: "arginine",
        one_letter: 'R',
        three_letter: "arg",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: Some(true),
        acidic: Some(false),
    },
    AminoAcid {
        name: "serine",
        one_letter: 'S',
        three_letter: "ser",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "threonine",
        one_letter: 'T',
        three_letter: "thr",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "valine",
        one_letter: 'V',
        three_letter: "val",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: Some(false),
        acidic: None,
    },
    AminoAcid {
        name: "tryptophan",
        one_letter: 'W',
        three_letter: "trp",
        side_chain_property: SideChainProperty::Nonpolar,
        hydrophilic: None,
        acidic: None,
    },
    AminoAcid {
        name: "tyrosine",
        one_letter: 'Y',
        three_letter: "tyr",
        side_chain_property: SideChainProperty::Polar,
        hydrophilic: None,
        acidic: None,
    },
];

pub fn amino_acid_by_one_letter(letter: char) -> Option<&'static AminoAcid> {
    AA_MAP.iter().find(|aa| aa.one_letter == letter)
}

pub fn amino_acid_by_three_letter(code: &str) -> Option<&'static AminoAcid> {
    AA_MAP
        .iter()
        .find(|aa| aa.three_letter.eq_ignore_ascii_case(code))
}
