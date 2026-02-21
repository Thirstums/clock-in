use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

const ROAST_CHANCE: f64 = 0.12; // 12%
const COOLDOWN_SECS: u64 = 45;

struct Handler {
    last_roast: Arc<Mutex<HashMap<u64, Instant>>>,
    roasts: Vec<&'static str>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }

 async fn message(&self, ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    }

    let content_lc = msg.content.to_lowercase();

    // --- Mention behavior: controlled responses ---
    if msg.mentions_me(&ctx.http).await.unwrap_or(false) {
        // Determine if this is a roast command
        let is_roast_cmd = content_lc.contains("roast");

        // Greeting vibes
        let is_greeting = content_lc.contains("hi")
            || content_lc.contains("hello")
            || content_lc.contains("hey")
            || content_lc.contains("yo")
            || content_lc.contains("sup")
            || content_lc.contains("whats up")
            || content_lc.contains("what's up");

        if is_roast_cmd {
            // Roast target: first mentioned user that's NOT the bot.
            // If no valid target, roast the author.
            let target = msg
                .mentions
                .iter()
                .find(|u| !u.bot) // typically skips your bot mention
                .unwrap_or(&msg.author);

            // cooldown per target to avoid spam roast-pings
            let target_id = target.id.get();
            {
                let mut map = self.last_roast.lock().await;
                if let Some(last) = map.get(&target_id) {
                    if last.elapsed() < Duration::from_secs(COOLDOWN_SECS) {
                        let _ = msg
                            .reply(&ctx.http, "Cool it. My roast cannon is on cooldown. 🙄")
                            .await;
                        return;
                    }
                }
                map.insert(target_id, Instant::now());
            }

            let idx = (rand::random::<f64>() * self.roasts.len() as f64) as usize;
            let roast = self.roasts[idx];

            let _ = msg
                .channel_id
                .say(&ctx.http, format!("{} {}", target.mention(), roast))
                .await;

            return;
        }

        if is_greeting {
            let nice_rude = [
                "hey. what do you want?",
                "hi. make it quick.",
                "yo. speak.",
                "hello. proceed with purpose.",
                "sup. try not to waste my time.",
                "hey bestie. state your business.",
            ];
            let idx = (rand::random::<f64>() * nice_rude.len() as f64) as usize;

            let _ = msg
                .reply(&ctx.http, nice_rude[idx])
                .await;

            return;
        }

        // Default mention response
        let defaults = [
            "yes?",
            "what do you want?",
            "you summoned me. now talk.",
            "i’m listening. unfortunately.",
            "make it interesting.",
        ];
        let idx = (rand::random::<f64>() * defaults.len() as f64) as usize;

        let _ = msg.reply(&ctx.http, defaults[idx]).await;
        return;
    }

    // --- Random roast behavior: chaotic background mode ---
    if msg.content.trim().len() < 3 {
        return;
    }

    let roll: f64 = rand::random();
    if roll > ROAST_CHANCE {
        return;
    }

    // Cooldown per author (so it doesn't bully one person nonstop)
    let user_id = msg.author.id.get();
    {
        let mut map = self.last_roast.lock().await;
        if let Some(last) = map.get(&user_id) {
            if last.elapsed() < Duration::from_secs(COOLDOWN_SECS) {
                return;
            }
        }
        map.insert(user_id, Instant::now());
    }

    let idx = (rand::random::<f64>() * self.roasts.len() as f64) as usize;
    let roast = self.roasts[idx];

    let _ = msg
        .reply(&ctx.http, format!("{} {}", msg.author.mention(), roast))
        .await;
}
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN in .env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        last_roast: Arc::new(Mutex::new(HashMap::new())),
        roasts: vec![
            "that message had the confidence of a lion and the accuracy of a broken clock.",
            "you typed that like you wanted to be wrong in public on purpose.",
            "respectfully, that was a side quest nobody asked for.",
            "your keyboard deserves better.",
            "this is why the 'edit' button exists.",
            "I’ve seen better takes in a microwave.",
                "that message had main character energy with NPC logic.",
            "you typed that like confidence is a substitute for accuracy.",
            "this feels like a thought that escaped containment.",
            "you said that like the internet wouldn’t respond.",
            "that take expired before you posted it.",
            "you’re arguing with the confidence of someone who didn’t fact-check.",
            "this message is running on vibes and nothing else.",
            "you pressed send like there were no consequences.",
            "that was ambitious. accuracy was optional though.",
            "you really built that opinion from scratch and still missed.",
            "that’s not a hot take, it’s a microwave malfunction.",
            "this message feels handcrafted from zero context.",
            "you typed that with full battery and still low power.",
            "this is premium confidence, economy reasoning.",
            "that was less of a statement and more of a suggestion.",
            "you really committed to that bit.",
            "this message brought a ladder for that reach.",
            "that thought should’ve stayed in airplane mode.",
            "you wrote that like you're speedrunning misunderstandings.",
            "this take needs a software update.",
            "you hit send like it was a dare.",
            "that opinion is buffering.",
            "this message feels like a deleted scene from common sense.",
            "you typed that like it was peer-reviewed.",
            "that’s not incorrect, it’s just aggressively imaginative.",
            "this take is under maintenance.",
            "you’re not out of pocket, you’re in a different zip code.",
            "that message had zero warmup and maximum delivery.",
            "you said that like context isn’t real.",
            "this is why we read past the first paragraph.",
            "that take is doing parkour away from logic.",
            "you really went all-in on that one.",
            "this feels like a trial version of a thought.",
            "you typed that like it couldn’t be challenged.",
            "that message is running on demo mode.",
            "you sound confident. that’s about it.",
            "this take is sponsored by guesswork.",
            "you really said that with your whole WiFi connection.",
            "that was bold. unnecessarily bold.",
            "this message skipped the tutorial.",
            "you wrote that like it was a mic drop.",
            "that’s a limited edition opinion. thankfully.",
            "this feels like a creative writing exercise.",
            "you typed that like citations are optional.",
            "that message took a shortcut through logic.",
            "you really leaned into that one.",
            "this take is experimental at best.",
            "you posted that like it couldn’t age badly.",
            "that’s not wrong, it’s just wildly ambitious.",
            "this message came with confidence pre-installed.",
            "you typed that like hindsight doesn’t exist.",
            "that was less insight, more inside joke with yourself.",
            "you hit send like you were proud of it.",
            "this take is operating outside recommended settings.",
            "you really committed to that misunderstanding.",
            "that opinion has strong beta energy.",
            "this message is doing its own thing. mostly wrong, but its own thing.",
            "you typed that like reality is optional.",
            "that was a thought. technically.",
            "this take needs adult supervision.",
            "you posted that like you expected a standing ovation.",
            "that’s not a hill to die on. that’s a speed bump.",
            "this message has strong 'first draft' energy.",
            "you really went freestyle with facts.",
            "that take is allergic to nuance.",
            "this feels like a confident guess.",
            "you typed that like you’ve never been corrected before.",
            "that opinion has vibes but no foundation.",
            "this message is stretching before the logic does.",
            "you said that like evidence is decorative.",
            "that take just respawned from nowhere.",
            "this is advanced typing with beginner reasoning.",
            "you pressed send and hoped for the best.",
            "that’s a brave place to stand with that logic.",
            "this message feels handcrafted from assumptions.",
            "you typed that like you expected backup.",
            "that take is doing backflips away from facts.",
            "this message has chaotic neutral energy.",
            "you really rolled the dice on that one.",
            "that opinion came with no warranty.",
            "this take is giving early access vibes.",
            "you posted that like the internet wouldn’t notice.",
            "that message is running on pure optimism.",
            "you typed that like context was DLC.",
            "this is a bold strategy for someone so loud.",
            "that take just invented a new genre of wrong.",
            "you said that like you rehearsed it.",
            "this message needs a patch update.",
            "you typed that like it was final boss logic.",
            "that was a creative interpretation of reality.",
            "this take is living in its own timeline.",
            "you really committed to the misunderstanding arc.",
            "that message is 90% confidence, 10% clarity.",
            "this feels like a plot twist nobody asked for.",
            "you typed that like it couldn’t possibly backfire.",
            "that opinion has strong 'I’ll defend this forever' energy.",
            "this message took the scenic route past logic.",
        ],
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}