//! Telegram bot integration for real-time performance monitoring.
//!
//! This module provides notifications for arbitrage opportunities,
//! trade executions, and system status updates via Telegram.

use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

/// Telegram Bot configuration
#[derive(Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
    pub enabled: bool,
}

impl TelegramConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Option<Self> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").ok()?;
        let chat_id = std::env::var("TELEGRAM_CHAT_ID").ok()?;
        let enabled = std::env::var("TELEGRAM_ENABLED")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(true);

        Some(Self {
            bot_token,
            chat_id,
            enabled,
        })
    }
}

/// Types of notifications that can be sent
#[derive(Debug, Clone)]
pub enum TelegramNotification {
    /// Bot started
    BotStarted {
        mode: String,
        markets_count: usize,
    },
    /// Arbitrage opportunity detected
    OpportunityDetected {
        market: String,
        yes_price: u16,
        no_price: u16,
        profit_cents: i16,
        arb_type: String,
    },
    /// Trade executed
    TradeExecuted {
        market: String,
        contracts: i64,
        profit_cents: i16,
        success: bool,
        latency_ms: u64,
    },
    /// Periodic status update
    StatusUpdate {
        uptime_hours: f64,
        total_trades: u64,
        successful_trades: u64,
        total_profit_cents: i64,
        markets_monitored: usize,
    },
    /// Error occurred
    Error {
        message: String,
    },
    /// Bot stopped
    BotStopped {
        reason: String,
    },
}

/// Telegram message sender
#[derive(Clone)]
pub struct TelegramBot {
    config: TelegramConfig,
    client: Client,
    api_url: String,
}

#[derive(Serialize)]
struct SendMessageRequest<'a> {
    chat_id: &'a str,
    text: &'a str,
    parse_mode: &'static str,
}

impl TelegramBot {
    /// Create a new Telegram bot instance
    pub fn new(config: TelegramConfig) -> Self {
        let api_url = format!("https://api.telegram.org/bot{}/sendMessage", config.bot_token);
        Self {
            config,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            api_url,
        }
    }

    /// Send a message to Telegram
    pub async fn send_message(&self, text: &str) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let request = SendMessageRequest {
            chat_id: &self.config.chat_id,
            text,
            parse_mode: "HTML",
        };

        let response = self.client
            .post(&self.api_url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("[TELEGRAM] Failed to send message: {}", error_text);
        }

        Ok(())
    }

    /// Format and send a notification
    pub async fn notify(&self, notification: TelegramNotification) -> Result<()> {
        let message = self.format_notification(notification);
        self.send_message(&message).await
    }

    /// Format notification into a readable message
    fn format_notification(&self, notification: TelegramNotification) -> String {
        match notification {
            TelegramNotification::BotStarted { mode, markets_count } => {
                format!(
                    "🚀 <b>Bot Arbitrage Démarré</b>\n\n\
                    📊 Mode: <code>{}</code>\n\
                    🎯 Marchés surveillés: <b>{}</b>\n\
                    ⏰ Heure: {}",
                    mode,
                    markets_count,
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                )
            }

            TelegramNotification::OpportunityDetected {
                market,
                yes_price,
                no_price,
                profit_cents,
                arb_type,
            } => {
                let profit_pct = profit_cents as f64 / 100.0;
                format!(
                    "🎯 <b>Opportunité Détectée!</b>\n\n\
                    📈 Marché: <code>{}</code>\n\
                    💰 YES: {}¢ | NO: {}¢\n\
                    💵 Profit: <b>{}¢ ({:.2}%)</b>\n\
                    🔄 Type: {}",
                    market,
                    yes_price,
                    no_price,
                    profit_cents,
                    profit_pct,
                    arb_type
                )
            }

            TelegramNotification::TradeExecuted {
                market,
                contracts,
                profit_cents,
                success,
                latency_ms,
            } => {
                let emoji = if success { "✅" } else { "❌" };
                let status = if success { "SUCCÈS" } else { "ÉCHEC" };
                format!(
                    "{} <b>Trade {}</b>\n\n\
                    📈 Marché: <code>{}</code>\n\
                    📦 Contrats: <b>{}</b>\n\
                    💵 Profit: <b>{}¢</b>\n\
                    ⚡ Latence: {}ms",
                    emoji,
                    status,
                    market,
                    contracts,
                    profit_cents,
                    latency_ms
                )
            }

            TelegramNotification::StatusUpdate {
                uptime_hours,
                total_trades,
                successful_trades,
                total_profit_cents,
                markets_monitored,
            } => {
                let success_rate = if total_trades > 0 {
                    (successful_trades as f64 / total_trades as f64) * 100.0
                } else {
                    0.0
                };
                let profit_dollars = total_profit_cents as f64 / 100.0;
                format!(
                    "📊 <b>Rapport de Statut</b>\n\n\
                    ⏱ Uptime: <b>{:.1}h</b>\n\
                    🎯 Marchés: {}\n\
                    📈 Trades: {}/{} ({:.1}% succès)\n\
                    💰 Profit Total: <b>${:.2}</b>\n\
                    ⏰ {}",
                    uptime_hours,
                    markets_monitored,
                    successful_trades,
                    total_trades,
                    success_rate,
                    profit_dollars,
                    chrono::Local::now().format("%H:%M:%S")
                )
            }

            TelegramNotification::Error { message } => {
                format!(
                    "⚠️ <b>Erreur Détectée</b>\n\n\
                    <code>{}</code>\n\
                    ⏰ {}",
                    message,
                    chrono::Local::now().format("%H:%M:%S")
                )
            }

            TelegramNotification::BotStopped { reason } => {
                format!(
                    "🛑 <b>Bot Arrêté</b>\n\n\
                    📝 Raison: {}\n\
                    ⏰ {}",
                    reason,
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                )
            }
        }
    }
}

/// Statistics tracker for performance monitoring
pub struct PerformanceTracker {
    start_time: Instant,
    pub total_trades: u64,
    pub successful_trades: u64,
    pub total_profit_cents: i64,
    pub opportunities_detected: u64,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_trades: 0,
            successful_trades: 0,
            total_profit_cents: 0,
            opportunities_detected: 0,
        }
    }

    pub fn record_trade(&mut self, success: bool, profit_cents: i16) {
        self.total_trades += 1;
        if success {
            self.successful_trades += 1;
            self.total_profit_cents += profit_cents as i64;
        }
    }

    pub fn record_opportunity(&mut self) {
        self.opportunities_detected += 1;
    }

    pub fn uptime_hours(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() / 3600.0
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Channel for sending notifications
pub type TelegramChannel = mpsc::Sender<TelegramNotification>;

/// Create a notification channel and spawn the sender loop
pub fn create_telegram_channel(bot: TelegramBot) -> TelegramChannel {
    let (tx, mut rx) = mpsc::channel::<TelegramNotification>(100);

    tokio::spawn(async move {
        while let Some(notification) = rx.recv().await {
            if let Err(e) = bot.notify(notification).await {
                error!("[TELEGRAM] Failed to send notification: {}", e);
            }
            // Rate limiting: max 1 message per second
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    tx
}

/// Optional wrapper for Telegram channel
#[derive(Clone)]
pub struct TelegramNotifier {
    channel: Option<TelegramChannel>,
}

impl TelegramNotifier {
    pub fn new(channel: Option<TelegramChannel>) -> Self {
        Self { channel }
    }

    pub fn none() -> Self {
        Self { channel: None }
    }

    /// Send a notification (non-blocking)
    pub fn notify(&self, notification: TelegramNotification) {
        if let Some(ref tx) = self.channel {
            let _ = tx.try_send(notification);
        }
    }

    /// Check if Telegram is enabled
    pub fn is_enabled(&self) -> bool {
        self.channel.is_some()
    }
}
