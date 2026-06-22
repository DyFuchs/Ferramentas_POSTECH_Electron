use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::Path;

// ============ ESTRUTURAS ============

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoInfo {
    pub nome: String,
    pub nome_original: String,
    pub aula: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrupoAula {
    pub numero: u32,
    pub titulo: String,
    pub videos: Vec<String>,
    pub num_videos: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigApp {
    #[serde(alias = "lastUsedPath")]
    pub last_used_path: Option<String>,
    #[serde(alias = "confirmRequired")]
    pub confirm_required: Option<bool>,
    #[serde(alias = "autoExtra")]
    pub auto_extra: Option<bool>,
    #[serde(alias = "fixedDest")]
    pub fixed_dest: Option<String>,
    #[serde(alias = "theme")]
    pub theme: Option<String>,
    #[serde(alias = "fontSize")]
    pub font_size: Option<String>,
    #[serde(alias = "fontWeight")]
    pub font_weight: Option<String>,
    #[serde(alias = "fontStyle")]
    pub font_style: Option<String>,
    #[serde(alias = "shortcuts")]
    pub shortcuts: Option<serde_json::Value>,
    #[serde(alias = "autoSaveMinutos")]
    pub auto_save_minutos: Option<i64>,
    #[serde(alias = "activeProfile")]
    pub active_profile: Option<String>,
    #[serde(alias = "colorProfiles")]
    pub color_profiles: Option<serde_json::Value>,
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            last_used_path: None,
            confirm_required: Some(true),
            auto_extra: Some(true),
            fixed_dest: None,
            theme: Some("dark".to_string()),
            font_size: Some("14".to_string()),
            font_weight: Some("700".to_string()),
            font_style: Some("normal".to_string()),
            shortcuts: None,
            auto_save_minutos: Some(10),
            active_profile: Some("POSTECH Escuro".to_string()),
            color_profiles: None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Command {
    #[serde(rename = "listarVideos")]
    ListarVideos { caminho: String },
    #[serde(rename = "agruparPorAula")]
    AgruparPorAula { videos: Vec<VideoInfo> },
    #[serde(rename = "organizarArquivos")]
    OrganizarArquivos {
        caminho: String,
        modo: String,
        criarExtras: bool,
    },
    #[serde(rename = "reverterOrganizacao")]
    ReverterOrganizacao {
        caminho: String,
        deletarRelatorios: bool,
    },
    #[serde(rename = "gerarCsv")]
    GerarCsv {
        caminho: String,
        grupos: Vec<GrupoAula>,
        titulos: Vec<String>,
    },
    #[serde(rename = "salvarConfig")]
    SalvarConfig { config: ConfigApp },
    #[serde(rename = "lerConfig")]
    LerConfig,
    #[serde(rename = "copiarParaClipboard")]
    CopiarParaClipboard { texto: String },
        #[serde(rename = "salvarRelatorio")]
    SalvarRelatorio { caminho: String, conteudo: String },
    #[serde(rename = "salvarProjeto")]
    SalvarProjeto { nome: String, dados: serde_json::Value },
    #[serde(rename = "carregarProjeto")]
    CarregarProjeto { nome: String },
    #[serde(rename = "listarProjetos")]
    ListarProjetos,
    #[serde(rename = "deletarProjeto")]
    DeletarProjeto { nome: String },
    #[serde(rename = "renomearProjeto")]
    RenomearProjeto { nome_antigo: String, nome_novo: String },
    #[serde(rename = "listarArquivos")]
    ListarArquivos { caminho: String, padrao: String },
    #[serde(rename = "salvarRelatorioHtml")]
    SalvarRelatorioHtml { caminho: String, conteudo: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum Response {
    #[serde(rename = "success")]
    Success { id: f64, data: serde_json::Value },
    #[serde(rename = "error")]
    Error { id: f64, message: String },
}

// ============ FUNÇÕES AUXILIARES ============

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "flv", "webm"];

fn is_video_file(path: &Path) -> bool {
    path.extension()
        .map(|e| {
            let ext = e.to_string_lossy().to_lowercase();
            VIDEO_EXTENSIONS.contains(&ext.as_str())
        })
        .unwrap_or(false)
}

fn extrair_numero_aula(nome: &str) -> u32 {
    let re = regex::Regex::new(r"[Aa]ula\s*(\d+)").unwrap();
    re.captures(nome)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0)
}

fn format_video_name(nome: &str) -> String {
    let mut result = nome.to_string();
    if let Some(idx) = result.rfind('/') {
        result = result[idx + 1..].to_string();
    }
    if let Some(idx) = result.rfind('\\') {
        result = result[idx + 1..].to_string();
    }
    if let Some(idx) = result.rfind('.') {
        result = result[..idx].to_string();
    }
    let re = regex::Regex::new(r"[\s_-]*[Vv]_?\d+\s*$").unwrap();
    result = re.replace(&result, "").to_string();
    let re2 = regex::Regex::new(r"[\s_-]+\d+\s*$").unwrap();
    result = re2.replace(&result, "").to_string();
    result
        .trim_end_matches(|c: char| c == ' ' || c == '-' || c == '_')
        .trim_start_matches(|c: char| c == ' ' || c == '-' || c == '_')
        .to_string()
}

fn send_response(id: f64, resp: Response) {
    let json = serde_json::to_string(&resp).unwrap();
    let mut stderr = io::stderr();
    stderr.write_all(json.as_bytes()).unwrap();
    stderr.write_all(b"\n").unwrap();
    stderr.flush().unwrap();
}




fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len()).position(|w| w == pattern)
}

fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

// ============ HANDLERS ============

fn handle_command(cmd: Command) {
    eprintln!("[RUST DEBUG] Iniciando handler");
    let result = std::panic::catch_unwind(|| match cmd {
        Command::ListarVideos { caminho } => match listar_videos(&caminho) {
            Ok(videos) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::to_value(videos).unwrap(),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::AgruparPorAula { videos } => match agrupar_por_aula(videos) {
            Ok(grupos) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::to_value(grupos).unwrap(),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::OrganizarArquivos {
            caminho,
            modo,
            criarExtras,
        } => match organizar_arquivos(&caminho, &modo, criarExtras) {
            Ok(result) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(result),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::ReverterOrganizacao {
            caminho,
            deletarRelatorios,
        } => match reverter_organizacao(&caminho, deletarRelatorios) {
            Ok(result) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(result),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::GerarCsv {
            caminho,
            grupos,
            titulos,
        } => match gerar_csv(&caminho, &grupos, &titulos) {
            Ok(result) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(result),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::SalvarConfig { config } => match salvar_config(&config) {
            Ok(_) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!("Config salva"),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::LerConfig => match ler_config() {
            Ok(config) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::to_value(config).unwrap(),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::CopiarParaClipboard { texto } => match copiar_para_clipboard(&texto) {
            Ok(_) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!("Copiado"),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::SalvarRelatorio { caminho, conteudo } => {
            let report_path = Path::new(&caminho).join("Relatorio_Organizacao.txt");
            match std::fs::write(&report_path, conteudo) {
                Ok(_) => send_response(0.0, Response::Success {
                    id: 0.0,
                    data: serde_json::json!(format!("Relatório salvo em: {}", report_path.display())),
                }),
                Err(e) => send_response(0.0, Response::Error { id: 0.0, message: format!("Erro ao salvar: {}", e) }),
            }
        },
        Command::SalvarProjeto { nome, dados } => match salvar_projeto(&nome, &dados) {
            Ok(path) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(format!("Projeto salvo: {}", path.display())),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::CarregarProjeto { nome } => match carregar_projeto(&nome) {
            Ok(dados) => send_response(0.0, Response::Success {
                id: 0.0,
                data: dados,
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::ListarProjetos => match listar_projetos() {
            Ok(projetos) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(projetos),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::DeletarProjeto { nome } => match deletar_projeto(&nome) {
            Ok(_) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!("Projeto deletado"),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::RenomearProjeto { nome_antigo, nome_novo } => match renomear_projeto(&nome_antigo, &nome_novo) {
            Ok(_) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!("Projeto renomeado"),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::ListarArquivos { caminho, padrao } => match listar_arquivos(&caminho, &padrao) {
            Ok(files) => send_response(0.0, Response::Success {
                id: 0.0,
                data: serde_json::json!(files),
            }),
            Err(e) => send_response(0.0, Response::Error { id: 0.0, message: e }),
        },
        Command::SalvarRelatorioHtml { caminho, conteudo } => {
            let html_path = std::path::Path::new(&caminho).join("Relatorio_Organizacao.html");
            match std::fs::write(&html_path, &conteudo) {
                Ok(_) => send_response(0.0, Response::Success {
                    id: 0.0,
                    data: serde_json::json!(format!("Relatorio HTML salvo em: {}", html_path.display())),
                }),
                Err(e) => send_response(0.0, Response::Error { id: 0.0, message: format!("Erro ao salvar HTML: {}", e) }),
            }
        },
    });
    match result {
        Ok(_) => eprintln!("[RUST DEBUG] Handler OK"),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown panic".to_string()
            };
            send_response(0.0, Response::Error {
                id: 0.0,
                message: format!("Panic: {}", msg),
            });
        }
    }
}

// ============ IMPLEMENTAÇÕES ============

fn listar_videos(caminho: &str) -> Result<Vec<VideoInfo>, String> {
    let path = Path::new(caminho);
    if !path.exists() {
        return Err(format!("Caminho não encontrado: {}", caminho));
    }
    if !path.is_dir() {
        return Err(format!("Não é uma pasta: {}", caminho));
    }
    let mut videos: Vec<VideoInfo> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() && is_video_file(&entry_path) {
                let nome = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let aula = extrair_numero_aula(&nome);
                videos.push(VideoInfo {
                    nome: nome.clone(),
                    nome_original: nome,
                    aula,
                });
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                if let Ok(sub_entries) = std::fs::read_dir(&entry_path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file() && is_video_file(&sub_path) {
                            let full_name = format!(
                                "{}/{}",
                                entry_path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy(),
                                sub_path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                            );
                            let aula = extrair_numero_aula(&full_name);
                            videos.push(VideoInfo {
                                nome: full_name.clone(),
                                nome_original: full_name,
                                aula,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(videos)
}

fn agrupar_por_aula(videos: Vec<VideoInfo>) -> Result<Vec<GrupoAula>, String> {
    let mut grupos: HashMap<u32, Vec<String>> = HashMap::new();
    for video in &videos {
        let aula_num = if video.aula == 0 { 1 } else { video.aula };
        grupos.entry(aula_num).or_default().push(video.nome.clone());
    }
    let mut grupo_vec: Vec<GrupoAula> = grupos
        .into_iter()
        .map(|(num, videos)| GrupoAula {
            numero: num,
            titulo: format!("Aula {}", num),
            num_videos: videos.len(),
            videos,
        })
        .collect();
    grupo_vec.sort_by_key(|g| g.numero);
    Ok(grupo_vec)
}

fn organizar_arquivos(caminho: &str, modo: &str, criarExtras: bool) -> Result<String, String> {
    let path = Path::new(caminho);
    if !path.exists() || !path.is_dir() {
        return Err("Caminho inválido".to_string());
    }
    let videos = listar_videos(caminho)?;
    let grupos = agrupar_por_aula(videos)?;
    let mut arquivos_movidos: usize = 0;
    let mut pastas_criadas: usize = 0;
    for grupo in &grupos {
        let pasta_aula = path.join(&grupo.titulo);
        if !pasta_aula.exists() {
            std::fs::create_dir_all(&pasta_aula)
                .map_err(|e| format!("Erro ao criar pasta {}: {}", pasta_aula.display(), e))?;
            pastas_criadas += 1;
        }
        for video_nome in &grupo.videos {
            let origem = path.join(video_nome);
            let destino = pasta_aula.join(
                std::path::Path::new(video_nome)
                    .file_name()
                    .unwrap_or_default(),
            );
            if origem.exists() && !destino.exists() {
                std::fs::rename(&origem, &destino).map_err(|e| {
                    format!(
                        "Erro ao mover {} → {}: {}",
                        origem.display(),
                        destino.display(),
                        e
                    )
                })?;
                arquivos_movidos += 1;
            }
        }
    }
    if criarExtras {
        for extra in &["Capítulo de Projeto", "Onboarding"] {
            let pasta_extra = path.join(extra);
            if !pasta_extra.exists() {
                std::fs::create_dir_all(&pasta_extra)
                    .map_err(|e| format!("Erro ao criar pasta extra {}: {}", extra, e))?;
                pastas_criadas += 1;
            }
        }
    }
    let modo_str = if modo == "auto" { "Automático" } else { "Manual" };
    Ok(format!(
        "{} concluído!\nArquivos movidos: {}\nPastas criadas: {}",
        modo_str, arquivos_movidos, pastas_criadas
    ))
}

fn reverter_organizacao(caminho: &str, deletarRelatorios: bool) -> Result<String, String> {
    let path = Path::new(caminho);
    if !path.exists() || !path.is_dir() {
        return Err("Caminho inválido".to_string());
    }
    let mut arquivos_movidos: usize = 0;
    let mut pastas_removidas: usize = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if !entry_path.is_dir() {
                continue;
            }
            let nome_pasta = entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let deve_processar = nome_pasta.starts_with("Aula ")
                || nome_pasta == "Capítulo de Projeto"
                || nome_pasta == "Onboarding";
            if deve_processar {
                if let Ok(sub_entries) = std::fs::read_dir(&entry_path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file() {
                            let destino = path.join(sub_path.file_name().unwrap_or_default());
                            if !destino.exists() {
                                std::fs::rename(&sub_path, &destino)
                                    .map_err(|e| format!("Erro ao mover arquivo: {}", e))?;
                                arquivos_movidos += 1;
                            }
                        }
                    }
                }
                if let Ok(remaining) = std::fs::read_dir(&entry_path) {
                    if remaining.count() == 0 {
                        std::fs::remove_dir(&entry_path)
                            .map_err(|e| format!("Erro ao remover pasta: {}", e))?;
                        pastas_removidas += 1;
                    }
                }
            }
        }
    }
    if deletarRelatorios {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(ext) = entry_path.extension() {
                        if ext == "txt" {
                            let _ = std::fs::remove_file(&entry_path);
                        }
                    }
                }
            }
        }
    }
    Ok(format!(
        "Reversão concluída!\nArquivos movidos para raiz: {}\nPastas removidas: {}",
        arquivos_movidos, pastas_removidas
    ))
}

fn gerar_csv(
    caminho: &str,
    grupos: &[GrupoAula],
    titulos: &[String],
) -> Result<String, String> {
    let path = Path::new(caminho);
    let mut csv_path = path.join("Planilha_Aulas.csv");
    let mut counter = 1;
    while csv_path.exists() {
        csv_path = path.join(format!("Planilha_Aulas ({}).csv", counter));
        counter += 1;
    }
    let mut conteudo = String::from("\u{FEFF}");
    conteudo.push_str("\"Aula\",\"Vídeo\"\r\n");
    for grupo in grupos {
        let titulo = if grupo.numero > 0 && (grupo.numero as usize) <= titulos.len() {
            &titulos[(grupo.numero - 1) as usize]
        } else {
            &grupo.titulo
        };
        for (i, video) in grupo.videos.iter().enumerate() {
            let nome_limpo = format_video_name(video);
            if i == 0 {
                conteudo.push_str(&format!("\"{}\",\"{}\"\r\n", titulo, nome_limpo));
            } else {
                conteudo.push_str(&format!("\"\",\"{}\"\r\n", nome_limpo));
            }
        }
    }
    std::fs::write(&csv_path, conteudo)
        .map_err(|e| format!("Erro ao salvar CSV: {}", e))?;
    Ok(format!("Planilha gerada: {}", csv_path.display()))
}


fn salvar_config(config: &ConfigApp) -> Result<(), String> {
    let config_path = get_config_path()?;
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Erro ao serializar: {}", e))?;
    std::fs::write(&config_path, json)
        .map_err(|e| format!("Erro ao salvar: {}", e))?;
    Ok(())
}

fn ler_config() -> Result<ConfigApp, String> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(ConfigApp::default());
    }
    let json = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Erro ao ler: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Erro ao parsear: {}", e))
}

fn copiar_para_clipboard(texto: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::io::Write;
        let mut child = Command::new("clip.exe")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Erro ao iniciar clip.exe: {}", e))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(texto.as_bytes())
                .map_err(|e| format!("Erro ao escrever no clipboard: {}", e))?;
            drop(stdin);
        }
        let output = child.wait_with_output()
            .map_err(|e| format!("Erro ao aguardar clip.exe: {}", e))?;
        if !output.status.success() {
            return Err(format!("clip.exe falhou com código: {:?}", output.status.code()));
        }
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        use std::io::Write;
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Erro ao copiar: {}", e))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(texto.as_bytes())
                .map_err(|e| format!("Erro ao escrever: {}", e))?;
            drop(stdin);
        }
        let output = child.wait_with_output()
            .map_err(|e| format!("Erro ao aguardar: {}", e))?;
        if !output.status.success() {
            return Err("pbcopy falhou".to_string());
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        use std::io::Write;
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Erro ao copiar: {}", e))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(texto.as_bytes())
                .map_err(|e| format!("Erro ao escrever: {}", e))?;
            drop(stdin);
        }
        let output = child.wait_with_output()
            .map_err(|e| format!("Erro ao aguardar: {}", e))?;
        if !output.status.success() {
            return Err("xclip falhou".to_string());
        }
    }
    Ok(())
}

fn get_config_path() -> Result<std::path::PathBuf, String> {
    let app_data = dirs::data_dir()
        .ok_or("Não foi possível obter diretório de dados")?;
    let config_dir = app_data.join("organizador-postech");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Erro ao criar diretório: {}", e))?;
    }
    Ok(config_dir.join("config.json"))
}

// ============ GERENCIAMENTO DE PROJETOS ============

fn get_projetos_dir() -> Result<std::path::PathBuf, String> {
    let app_data = dirs::data_dir()
        .ok_or("Não foi possível obter diretório de dados")?;
    let projetos_dir = app_data.join("organizador-postech").join("projetos");
    if !projetos_dir.exists() {
        std::fs::create_dir_all(&projetos_dir)
            .map_err(|e| format!("Erro ao criar diretório: {}", e))?;
    }
    Ok(projetos_dir)
}

fn salvar_projeto(nome: &str, dados: &serde_json::Value) -> Result<std::path::PathBuf, String> {
    let projetos_dir = get_projetos_dir()?;
    let nome_seguro = nome.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != ' ', "_");
    let path = projetos_dir.join(format!("{}.json", nome_seguro));
    let json = serde_json::to_string_pretty(dados)
        .map_err(|e| format!("Erro ao serializar: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Erro ao salvar: {}", e))?;
    Ok(path)
}

fn carregar_projeto(nome: &str) -> Result<serde_json::Value, String> {
    let projetos_dir = get_projetos_dir()?;
    let nome_seguro = nome.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != ' ', "_");
    let path = projetos_dir.join(format!("{}.json", nome_seguro));
    if !path.exists() {
        return Err(format!("Projeto '{}' não encontrado", nome));
    }
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("Erro ao ler: {}", e))?;
    serde_json::from_str(&json)
        .map_err(|e| format!("Erro ao parsear: {}", e))
}

fn listar_projetos() -> Result<Vec<String>, String> {
    let projetos_dir = get_projetos_dir()?;
    let mut projetos = Vec::new();
    if projetos_dir.exists() {
        for entry in std::fs::read_dir(&projetos_dir)
            .map_err(|e| format!("Erro ao ler diretório: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Erro ao ler entrada: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Some(nome) = path.file_stem().and_then(|s| s.to_str()) {
                    projetos.push(nome.to_string());
                }
            }
        }
    }
    projetos.sort();
    Ok(projetos)
}

fn deletar_projeto(nome: &str) -> Result<(), String> {
    let projetos_dir = get_projetos_dir()?;
    let nome_seguro = nome.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != ' ', "_");
    let path = projetos_dir.join(format!("{}.json", nome_seguro));
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Erro ao deletar: {}", e))?;
    }
    Ok(())
}

fn renomear_projeto(nome_antigo: &str, nome_novo: &str) -> Result<(), String> {
    let projetos_dir = get_projetos_dir()?;
    let antigo_seguro = nome_antigo.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != ' ', "_");
    let novo_seguro = nome_novo.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != ' ', "_");
    let path_antigo = projetos_dir.join(format!("{}.json", antigo_seguro));
    let path_novo = projetos_dir.join(format!("{}.json", novo_seguro));
    if !path_antigo.exists() {
        return Err(format!("Projeto '{}' não encontrado", nome_antigo));
    }
    if path_novo.exists() {
        return Err(format!("Já existe um projeto com o nome '{}'", nome_novo));
    }
    std::fs::rename(&path_antigo, &path_novo)
        .map_err(|e| format!("Erro ao renomear: {}", e))?;
    Ok(())
}

fn listar_arquivos(caminho: &str, padrao: &str) -> Result<Vec<String>, String> {
    let dir = std::path::Path::new(caminho);
    if !dir.exists() {
        return Err(format!("Pasta não encontrada: {}", caminho));
    }
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| format!("Erro ao ler pasta: {}", e))? {
        let entry = entry.map_err(|e| format!("Erro ao ler entrada: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if padrao == "*" || padrao.is_empty() {
            files.push(name);
        } else if padrao.starts_with("*.") {
            let ext = &padrao[1..];
            if name.ends_with(ext) {
                files.push(name);
            }
        } else if name.contains(padrao) {
            files.push(name);
        }
    }
    files.sort();
    Ok(files)
}

// ============ MAIN ============

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                eprintln!("[RUST DEBUG] Recebido: {}", line);
                match serde_json::from_str::<Command>(&line) {
                    Ok(cmd) => {
                        eprintln!("[RUST DEBUG] Comando parseado");
                        handle_command(cmd);
                    }
                    Err(e) => send_response(0.0, Response::Error {
                        id: 0.0,
                        message: format!("Erro ao parsear comando: {}", e),
                    }),
                }
            }
            Err(e) => {
                eprintln!("[ERRO] Falha ao ler stdin: {}", e);
                break;
            }
        }
    }
}
