use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, SystemTime};
use chrono::prelude::*;
use std::io;

const DIRETORIO_MONITORADO: &str = "/home/marcio/tmp/monitorado";
const DIRETORIO_DESTINO: &str = "/home/marcio/tmp/destino/a";

fn mover_e_renomear_arquivo(arquivo: &PathBuf, parametros: &Arquivo) {

	// Determina o horário atual por meio da biblioteca chrono
	let tempo: DateTime<Local> = Local::now();
	// Formata a saída do modo desejado, sendo que o %3.f indica o número de casas decimais do campo de segundos.
	let tempo_formatado = tempo.format_localized("%Y-%m-%d %H:%M:%S%.3f", Locale::pt_BR).to_string();
		
	let novo_nome = format!("{}_{}.{}", tempo_formatado, parametros.sufixo, parametros.extensao);
	
	let novo_caminho = PathBuf::from(DIRETORIO_DESTINO).with_file_name(novo_nome);
	
	println!("novo_caminho 3 {:?}", &novo_caminho);

	fs::rename(arquivo, &novo_caminho).expect("Falha ao mover e renomear o arquivo");
}

fn monitorar_diretorio(par: &Arquivo) {
	
	let mut tempos_modificacao: Vec<(PathBuf, SystemTime)> = Vec::new();

	loop {
			if let Ok(entries) = fs::read_dir(DIRETORIO_MONITORADO) {
					for entry in entries {
							if let Ok(entry) = entry {
									let arquivo = entry.path();
									if arquivo.is_file() {
											let metadata = fs::metadata(&arquivo).expect("Falha ao obter metadados do arquivo");
											let tempo_modificacao = metadata.modified().expect("Falha ao obter tempo de modificação do arquivo");

											if let Some(index) = tempos_modificacao.iter().position(|(path, _)| path == &arquivo) {
													let (_, tempo_anterior) = tempos_modificacao[index];
													if tempo_modificacao != tempo_anterior {
															mover_e_renomear_arquivo(&arquivo, &par);
															tempos_modificacao[index] = (arquivo.clone(), tempo_modificacao);
													}
											} else {
													tempos_modificacao.push((arquivo.clone(), tempo_modificacao));
													mover_e_renomear_arquivo(&arquivo, &par);
											}
									}
							}
					}
			}

			thread::sleep(Duration::from_secs(1));
	}
}

#[derive(Debug, Clone)]
struct Arquivo {
	sufixo: String,
	extensao: String,
}

impl Arquivo  {

	fn novo_arquivo() -> Arquivo {
	
		println!("\nDigite o sufixo para os nomes dos arquivos: ");
		let mut sufixo = String::new();
		io::stdin().read_line(&mut sufixo).expect("Erro ao informar o sufixo!");

			
		println!("Digite a extensão referente ao tipo de arquivo, sem o ponto. ");
		let mut extensao = String::new();

		io::stdin().read_line(&mut extensao).expect("Erro ao informar a extensão!");
		
		Arquivo {
			sufixo: sufixo.trim().to_string(),
			extensao: extensao.trim().to_string(),
		}
	}
	
}

fn main() {

	let par: Arquivo = Arquivo::novo_arquivo();

	monitorar_diretorio(&par);
}
