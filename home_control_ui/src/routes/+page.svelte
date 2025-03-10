<script>
    // You can add logic here later
    let rooms = [
        { name: "Living Room", icon: "🛋️" },
        { name: "Bedroom", icon: "🛏️" },
        { name: "Kitchen", icon: "🍳" },
        { name: "Bathroom", icon: "🛁", path: "/bathroom" },
        { name: "Outdoor", icon: "🌳" },
    ];
</script>

<main class="container">
    <section class="rooms">
        <h2>Rooms</h2>
        <div class="room-grid">
            {#each rooms as room}
                <a href={room.path || "#"} class="room-card">
                    <span class="room-icon">{room.icon}</span>
                    <h3>{room.name}</h3>
                </a>
            {/each}
        </div>
    </section>
    </main>

<style>
    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 20px;
        font-family: sans-serif;
    }

    .header {
        text-align: center;
        margin-bottom: 30px;
    }

    .rooms {
        margin-bottom: 30px;
    }

    .room-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 20px;
    }

    .room-card {
        background-color: #f0f0f0;
        padding: 20px;
        border-radius: 8px;
        text-align: center;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .room-card:hover {
        background-color: #e0e0e0;
    }

    .room-icon {
        font-size: 2rem;
        display: block;
        margin-bottom: 10px;
    }

    .quick-actions {
        margin-bottom: 30px;
    }

    .action-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 10px;
    }

    .action-button {
        padding: 15px 20px;
        border: none;
        border-radius: 8px;
        background-color: #007bff;
        color: white;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .action-button:hover {
        background-color: #0056b3;
    }
</style>