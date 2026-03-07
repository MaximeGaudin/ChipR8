use macroquad::audio::{load_sound_from_bytes, Sound};

pub async fn generate_beep() -> Sound {
    let sample_rate: u32 = 44100;
    let frequency: f32 = 440.0;
    let duration: f32 = 0.5;
    let num_samples = (sample_rate as f32 * duration) as usize;
    
    // 1. Générer les samples (Square wave)
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let val: i16 = if (t * frequency).fract() < 0.5 { 4000 } else { -4000 };
        samples.push(val);
    }

    // 2. Construire un header WAV minimal (44 octets)
    let mut wav_file = Vec::new();
    let data_size = (num_samples * 2) as u32; // 2 octets par sample (i16)
    
    wav_file.extend_from_slice(b"RIFF");
    wav_file.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav_file.extend_from_slice(b"WAVEfmt ");
    wav_file.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size
    wav_file.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat (PCM)
    wav_file.extend_from_slice(&1u16.to_le_bytes());  // NumChannels (Mono)
    wav_file.extend_from_slice(&sample_rate.to_le_bytes());
    wav_file.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // ByteRate
    wav_file.extend_from_slice(&2u16.to_le_bytes());  // BlockAlign
    wav_file.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample
    wav_file.extend_from_slice(b"data");
    wav_file.extend_from_slice(&data_size.to_le_bytes());

    // 3. Ajouter les données audio
    for sample in samples {
        wav_file.extend_from_slice(&sample.to_le_bytes());
    }

    // 4. Charger comme si c'était un fichier chargé du disque
    load_sound_from_bytes(&wav_file).await.expect("Failed to load generated WAV")
}