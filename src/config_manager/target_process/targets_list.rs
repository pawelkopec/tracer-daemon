use lazy_static::lazy_static;

use crate::config_manager::target_process::target_matching::{CommandContainsStruct, TargetMatch};

use super::Target;

lazy_static! {
    pub static ref TARGETS: Vec<Target> = [
        Target::new(TargetMatch::ProcessName("python".to_string())),
        Target::new(TargetMatch::CommandContains(CommandContainsStruct {
            process_name: Some("java".to_string()),
            command_content: "uk.ac.babraham.FastQC.FastQCApplication".to_string()
        }))
        .set_display_name(Some("fastqc".to_string()))
        .set_merge_with_parents(true)
        .set_force_ancestor_to_match(false),
        Target::new(TargetMatch::CommandContains(CommandContainsStruct {
            process_name: Some("bowtie2-build-s".to_string()),
            command_content: "/opt/conda/bin/bowtie2-build-s".to_string()
        }))
        .set_display_name(Some("bowtie2-build-s (Conda)".to_string())),
        Target::new(TargetMatch::ProcessName("STAR".to_string())),
        // bowtie section
        Target::new(TargetMatch::ProcessName("bowtie2".to_string())),
        Target::new(TargetMatch::ProcessName("bowtie2-build-s".to_string())),
        Target::new(TargetMatch::ProcessName("bowtie2-align-s".to_string())),
        Target::new(TargetMatch::ProcessName("bwa".to_string())),
        Target::new(TargetMatch::ProcessName("salmon".to_string())),
        Target::new(TargetMatch::ProcessName("hisat2".to_string())).set_display_name(Some("HISAT2".to_string())),
        Target::new(TargetMatch::ProcessName("hisat2-build".to_string())).set_display_name(Some("HISAT2_BUILD".to_string())),
        Target::new(TargetMatch::ProcessName("stringtie".to_string())),
        Target::new(TargetMatch::ProcessName("HOMER".to_string())),
        Target::new(TargetMatch::ProcessName("samtools".to_string())),
        Target::new(TargetMatch::ProcessName("bedtools".to_string())),
        Target::new(TargetMatch::ProcessName("deeptools".to_string())),
        Target::new(TargetMatch::ProcessName("macs3".to_string())),
        Target::new(TargetMatch::ProcessName("plotCoverage".to_string())),
        Target::new(TargetMatch::ProcessName("plotFingerprint".to_string())),
        Target::new(TargetMatch::ProcessName("MACS33".to_string())),
        Target::new(TargetMatch::ProcessName("Genrich".to_string())),
        Target::new(TargetMatch::ProcessName("TopHat".to_string())),
        Target::new(TargetMatch::ProcessName("JAMM".to_string())),
        Target::new(TargetMatch::ProcessName("fastqc".to_string())),
        Target::new(TargetMatch::ShortLivedProcessExecutable(
            "fastqc".to_string()
        )),
        Target::new(TargetMatch::ProcessName("multiqc".to_string())),
        Target::new(TargetMatch::ProcessName("fastp".to_string())),
        Target::new(TargetMatch::ProcessName("PEAR".to_string())),
        Target::new(TargetMatch::ProcessName("Trimmomatic".to_string())),
        Target::new(TargetMatch::ProcessName("sra-toolkit".to_string())),
        Target::new(TargetMatch::ProcessName("Picard".to_string())),
        Target::new(TargetMatch::ProcessName("cutadapt".to_string())),
        Target::new(TargetMatch::ProcessName("cellranger".to_string())),
        Target::new(TargetMatch::ProcessName("STATsolo".to_string())),
        Target::new(TargetMatch::ProcessName("scTE".to_string())),
        Target::new(TargetMatch::ProcessName("scanpy".to_string())),
        Target::new(TargetMatch::ProcessName("Seurat".to_string())),
        Target::new(TargetMatch::ProcessName("LIGER".to_string())),
        Target::new(TargetMatch::ProcessName("SC3".to_string())),
        Target::new(TargetMatch::ProcessName("Louvain".to_string())),
        Target::new(TargetMatch::ProcessName("Leiden".to_string())),
        Target::new(TargetMatch::ProcessName("Garnett".to_string())),
        Target::new(TargetMatch::ProcessName("Monocle".to_string())),
        Target::new(TargetMatch::ProcessName("Harmony".to_string())),
        Target::new(TargetMatch::ProcessName("PAGA".to_string())),
        Target::new(TargetMatch::ProcessName("Palantir".to_string())),
        Target::new(TargetMatch::ProcessName("velocity".to_string())),
        Target::new(TargetMatch::ProcessName("CellPhoneDB".to_string())),
        Target::new(TargetMatch::ProcessName("CellChat".to_string())),
        Target::new(TargetMatch::ProcessName("NicheNet".to_string())),
        Target::new(TargetMatch::ProcessName("FIt-SNE".to_string())),
        Target::new(TargetMatch::ProcessName("umap".to_string())),
        Target::new(TargetMatch::ProcessName("bbmap".to_string())),
        Target::new(TargetMatch::ProcessName("cuffdiff".to_string())),
        Target::new(TargetMatch::ProcessName("RNA-SeQC".to_string())),
        Target::new(TargetMatch::ProcessName("RSeQC".to_string())),
        Target::new(TargetMatch::ProcessName("Trimgalore".to_string())),
        Target::new(TargetMatch::ProcessName("UCHIME".to_string())),
        Target::new(TargetMatch::ProcessName("Erange".to_string())),
        Target::new(TargetMatch::ProcessName("X-Mate".to_string())),
        Target::new(TargetMatch::ProcessName("SpliceSeq".to_string())),
        Target::new(TargetMatch::ProcessName("casper".to_string())),
        Target::new(TargetMatch::ProcessName("DESeq".to_string())),
        Target::new(TargetMatch::ProcessName("EdgeR".to_string())),
        Target::new(TargetMatch::ProcessName("kallisto".to_string())),
        Target::new(TargetMatch::ProcessName("pairtools".to_string())),
        Target::new(TargetMatch::ProcessName("HiCExplorer".to_string())),
        Target::new(TargetMatch::ProcessName("GITAR".to_string())),
        Target::new(TargetMatch::ProcessName("TADbit".to_string())),
        Target::new(TargetMatch::ProcessName("Juicer".to_string())),
        Target::new(TargetMatch::ProcessName("HiC-Pro".to_string())),
        Target::new(TargetMatch::ProcessName("cooler".to_string())),
        Target::new(TargetMatch::ProcessName("cooltools".to_string())),
        Target::new(TargetMatch::ProcessName("runHiC".to_string())),
        Target::new(TargetMatch::ProcessName("HTSlib".to_string())),
        Target::new(TargetMatch::ProcessName("htslib".to_string())),
        Target::new(TargetMatch::ProcessName("zlib".to_string())),
        Target::new(TargetMatch::ProcessName("libbz2".to_string())),
        Target::new(TargetMatch::ProcessName("liblzma".to_string())),
        Target::new(TargetMatch::ProcessName("libcurl".to_string())),
        Target::new(TargetMatch::ProcessName("libdeflate".to_string())),
        Target::new(TargetMatch::ProcessName("ncurses".to_string())),
        Target::new(TargetMatch::ProcessName("pthread".to_string())),
    ]
    .to_vec();
}