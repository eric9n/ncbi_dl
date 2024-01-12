use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use ncbi::fna::write_to_fna;
use ncbi::meta::{init_meta, save_meta};
use ncbi::task;
use ncbi::utils;
use std::fmt;
use std::path::PathBuf;
use tokio::runtime::Builder;

const NCBI_LIBRARY: &'static [&str] = &[
    "archaea", "bacteria", "viral", "fungi", "plant", "human", "protozoa",
];

fn validate_group(group: &str) -> Result<String, String> {
    let groups = utils::parse_comma_separated_list(&group);
    for grp in &groups {
        if !NCBI_LIBRARY.contains(&grp.as_str()) {
            return Err(format!("group not in ncbi library"));
        }
    }
    Ok(group.to_string())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Site {
    /// 下载 genbank 资源
    Genbank,
    /// 下载 refseq 资源
    Refseq,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Site::Genbank => "genbank",
                Site::Refseq => "refseq",
            }
        )
    }
}

#[derive(Subcommand, Debug)]
enum Mode {
    /// 仅检查文件的 md5
    Md5,
    /// 解析 genomic 文件，并且生成 library fna 文件
    Fna,
}

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "ncbi download resource",
    long_about = "从 ncbi 网站上下载 genomes 资源"
)]
struct Args {
    /// 构建数据库的目录
    #[arg(short, long, default_value = "lib")]
    database: PathBuf,

    /// 下载时的并行大小
    #[arg(short, long, default_value = "8")]
    num_threads: usize,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 从 NCBI 下载 taxonomy 文件 (alias: tax)
    #[command(alias = "tax")]
    Taxonomy,

    /// 从 NCBI 下载 genomes 数据 (alias: gen)
    #[command(alias = "gen")]
    Genomes {
        /// 从 NCBI 哪个站点目录下载（RefSeq或GenBank）
        #[arg(long, value_enum, default_value_t = Site::Refseq)]
        site: Site,

        /// 从 NCBI 站点上下载某个种类的数据信息，可以是逗号分隔的多个, archaea,bacteria,viral,fungi,plant,human,protozoa
        #[arg(short, long, value_parser = validate_group)]
        group: String,

        /// 子命令，使用 md5 校验和生成 fna 文件
        #[command(subcommand)]
        mode: Option<Mode>,
    },
}

async fn async_run(args: Args) -> Result<()> {
    let db_path = utils::create_data_dir(&args.database).unwrap();
    init_meta(&db_path).await;

    match args.command {
        Commands::Taxonomy => {
            let data_dir: PathBuf = db_path.join("taxonomy");
            utils::create_dir(&data_dir)?;
            let _ = task::run_taxo(&data_dir).await;
        }
        Commands::Genomes { site, group, mode } => {
            let site = site.to_string();
            let groups = utils::parse_comma_separated_list(&group);
            for grp in groups {
                let data_dir: PathBuf = db_path.join("library").join(grp.clone());
                utils::create_dir(&data_dir.join(&site))?;

                let trans_group = if &grp == "human" {
                    "vertebrate_mammalian/Homo_sapiens".to_string()
                } else {
                    grp.to_string()
                };

                match &mode {
                    Some(Mode::Md5) => {
                        let _ =
                            task::run_check(&site, &trans_group, &data_dir, args.num_threads).await;
                    }
                    Some(Mode::Fna) => {
                        let _ = write_to_fna(&site, &trans_group, &data_dir).await;
                    }
                    None => {
                        let _ =
                            task::run_task(&site, &trans_group, &data_dir, args.num_threads).await;
                    }
                }
            }
        }
    }

    save_meta(&db_path).await?;
    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();
    let num_thread = args.num_threads.clone();
    // 创建一个 Runtime 实例，并配置线程数
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .thread_name("ncbi")
        // .max_blocking_threads(100)
        .worker_threads(num_thread) // 设置所需的工作线程数
        .build()
        .expect("Failed to create runtime");

    // 使用 Runtime 运行异步代码
    runtime.block_on(async_run(args))?;

    Ok(())
}
