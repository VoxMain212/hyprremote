let ws;
const statusEl = document.getElementById('status');
const logEl = document.getElementById('log');

function connect() {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const params = new URLSearchParams(location.search);
    const token = params.get('token') || '';
    const url = `${proto}//${location.host}/ws?token=${token}`;
    
    ws = new WebSocket(url);
    
    ws.onopen = () => {
        statusEl.textContent = '🟢 Подключено';
        log('Соединение установлено');
    };
    
    ws.onclose = () => {
        statusEl.textContent = '🔴 Отключено';
        log('Соединение закрыто');
        setTimeout(connect, 3000);
    };
    
    ws.onerror = () => {
        statusEl.textContent = '⚠️ Ошибка';
        log('Ошибка соединения');
    };
    
    ws.onmessage = (e) => {
        try {
            const msg = JSON.parse(e.data);
            log(`${msg.action || 'Message'}: ${msg.status} - ${msg.output || msg.message || ''}`);
        } catch {
            log(e.data);
        }
    };
}

connect();

function send(action, params = {}) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ action, params }));
    } else {
        log('Нет соединения');
    }
}

function log(msg) {
    if (logEl) {
        logEl.textContent = msg;
    }
    console.log(msg);
}

// Трекпад (относительное перемещение)
const trackpad = document.getElementById('trackpad');
let lastTouch = null;

if (trackpad) {
    trackpad.addEventListener('touchstart', (e) => {
        e.preventDefault();
        lastTouch = e.touches[0];
    });
    
    trackpad.addEventListener('touchmove', (e) => {
        e.preventDefault();
        if (!lastTouch) return;
        const touch = e.touches[0];
        const dx = touch.clientX - lastTouch.clientX;
        const dy = touch.clientY - lastTouch.clientY;
        send('mouse_move', { dx, dy });
        lastTouch = touch;
    });
    
    trackpad.addEventListener('touchend', () => {
        lastTouch = null;
    });
    
    // Для тестирования мышью
    trackpad.addEventListener('mousedown', (e) => {
        lastTouch = e;
    });
    
    trackpad.addEventListener('mousemove', (e) => {
        if (!lastTouch) return;
        const dx = e.clientX - lastTouch.clientX;
        const dy = e.clientY - lastTouch.clientY;
        send('mouse_move', { dx, dy });
        lastTouch = e;
    });
    
    trackpad.addEventListener('mouseup', () => {
        lastTouch = null;
    });
}

// Мышиные кнопки
document.getElementById('btn-left')?.addEventListener('click', () => {
    send('mouse_click', { button: 'left' });
});

document.getElementById('btn-right')?.addEventListener('click', () => {
    send('mouse_click', { button: 'right' });
});

document.getElementById('btn-middle')?.addEventListener('click', () => {
    send('mouse_click', { button: 'middle' });
});

// Рабочие столы через data-keys
document.querySelectorAll('.key-combo').forEach(btn => {
    btn.addEventListener('click', () => {
        const keys = btn.dataset.keys.split(',');
        send('key_combo', { keys });
    });
});

// Громкость
document.getElementById('vol-up')?.addEventListener('click', () => {
    send('volume', { mode: 'up' });
});

document.getElementById('vol-down')?.addEventListener('click', () => {
    send('volume', { mode: 'down' });
});

document.getElementById('vol-mute')?.addEventListener('click', () => {
    send('volume', { mode: 'mute' });
});

document.getElementById('vol-slider')?.addEventListener('input', (e) => {
    send('volume', { mode: `set ${e.target.value}%` });
});

// Яркость
document.getElementById('bl-up')?.addEventListener('click', () => {
    send('backlight', { op: 'up' });
});

document.getElementById('bl-down')?.addEventListener('click', () => {
    send('backlight', { op: 'down' });
});

document.getElementById('bl-slider')?.addEventListener('input', (e) => {
    send('backlight', { op: `set ${e.target.value}%` });
});

// Медиа
document.getElementById('media-prev')?.addEventListener('click', () => {
    send('media', { cmd: 'previous' });
});

document.getElementById('media-play')?.addEventListener('click', () => {
    send('media', { cmd: 'play-pause' });
});

document.getElementById('media-next')?.addEventListener('click', () => {
    send('media', { cmd: 'next' });
});

// Командная строка
document.getElementById('cmd-run')?.addEventListener('click', () => {
    const cmdInput = document.getElementById('cmd-input');
    if (cmdInput) {
        const cmd = cmdInput.value.trim();
        if (cmd) {
            send('exec', { command: cmd });
            cmdInput.value = '';
        }
    }
});