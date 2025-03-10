<script>
    let intensity = 22;
    let bulbOpacity = intensity / 100; // Opacity based on intensity
    let bulbGlow = `0 0 ${intensity / 5}px rgba(255, 255, 200, ${bulbOpacity})`; // Glow effect

    async function sendIntensity() {
        try {
            const response = await fetch(`http://192.168.1.137:5123/bathroom_led?intensity=${intensity}`);

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const result = await response.json();
            console.log(result.message); // Optional: Log the API response
        } catch (error) {
            console.error('Error sending intensity:', error);
        }
    }

</script>

<main class="bathroom">
    <h1>Bathroom LED Control</h1>

    <div class="controls">
        <div class="led-control">
            <h2>LED Intensity</h2>
            <div class="intensity-display">
                <div class="light-bulb" style="opacity: {bulbOpacity}; box-shadow: {bulbGlow};">
                    <img src="/lightbulb.svg" alt="Light Bulb" />
                </div>
                <p>{intensity}%</p>
            </div>
            <input type="range" min="0" max="100" bind:value={intensity} on:input={sendIntensity}/>
        </div>
    </div>

    <a href="/" class="back-button">Back to Dashboard</a>
</main>

<style>
    .bathroom {
        max-width: 600px;
        margin: 50px auto;
        padding: 30px;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        background: linear-gradient(135deg, #f0f0f0, #e0e0e0);
        border-radius: 15px;
        box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
    }

    h1 {
        text-align: center;
        margin-bottom: 30px;
        color: #333;
    }

    .controls {
        display: flex;
        flex-direction: column;
        gap: 30px;
    }

    .led-control {
        background-color: white;
        padding: 30px;
        border-radius: 12px;
        box-shadow: 0 3px 10px rgba(0, 0, 0, 0.08);
    }

    .intensity-display {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 20px;
    }

    .light-bulb {
        width: 80px;
        height: 80px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: opacity 0.3s ease, box-shadow 0.3s ease;
    }

    .light-bulb img {
        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .intensity-display p {
        font-size: 1.2rem;
        color: #555;
    }

    .led-control input[type="range"] {
        width: 100%;
        -webkit-appearance: none;
        height: 8px;
        border-radius: 5px;
        background: #ddd;
        outline: none;
        -webkit-transition: 0.2s;
        transition: opacity 0.2s;
    }

    .led-control input[type="range"]::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        background: #007bff;
        cursor: pointer;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    }

    .back-button {
        display: block;
        text-align: center;
        margin-top: 30px;
        padding: 12px 25px;
        background-color: #007bff;
        color: white;
        text-decoration: none;
        border-radius: 8px;
        transition: background-color 0.3s ease;
    }

    .back-button:hover {
        background-color: #0056b3;
    }
</style>