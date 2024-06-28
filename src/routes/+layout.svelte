<script>
    import { ModeWatcher, setMode } from "mode-watcher";
    import "../app.css";

    setMode("dark"); // lmao
</script>

<ModeWatcher />

<div class="app">
    <main>
        <slot></slot>
    </main>
</div>
