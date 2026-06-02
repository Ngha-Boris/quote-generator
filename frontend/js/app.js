// ══════════════════════════════════════════════════════════════
// Quote Generator — Frontend Application Logic
// ══════════════════════════════════════════════════════════════

const API_BASE = '/api';
let quoteHistory = [];
let currentQuote = null;

// ── DOM Elements ────────────────────────────────────────────
const quoteText = document.getElementById('quoteText');
const quoteAuthor = document.getElementById('quoteAuthor');
const quoteCard = document.getElementById('quoteCard');
const quoteShimmer = document.getElementById('quoteShimmer');
const generateBtn = document.getElementById('generateBtn');
const copyBtn = document.getElementById('copyBtn');
const statusDot = document.getElementById('statusDot');
const statusText = document.getElementById('statusText');
const historySection = document.getElementById('historySection');
const historyList = document.getElementById('historyList');

// ── Generate Quote ──────────────────────────────────────────
async function generateQuote() {
    setLoading(true);

    try {
        const response = await fetch(`${API_BASE}/quotes/random`);

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        const data = await response.json();
        currentQuote = data;

        // Animate the transition
        quoteCard.classList.add('loading');

        setTimeout(() => {
            quoteText.textContent = data.quote;
            quoteAuthor.textContent = `— ${data.author}`;
            quoteCard.classList.remove('loading');

            // Trigger shimmer
            quoteShimmer.classList.remove('active');
            void quoteShimmer.offsetWidth; // Force reflow
            quoteShimmer.classList.add('active');

            // Enable copy button
            copyBtn.disabled = false;

            // Add to history
            addToHistory(data);
        }, 200);

        setStatus('success', 'Quote loaded');
    } catch (error) {
        console.error('Failed to fetch quote:', error);
        setStatus('error', 'Connection failed');
        showToast('Failed to fetch quote. Is the server running?');
    } finally {
        setLoading(false);
    }
}

// ── Copy Quote ──────────────────────────────────────────────
async function copyQuote() {
    if (!currentQuote) return;

    const text = `"${currentQuote.quote}" — ${currentQuote.author}`;

    try {
        await navigator.clipboard.writeText(text);
        showToast('Quote copied to clipboard!');

        // Visual feedback
        const btnText = copyBtn.querySelector('.btn-text');
        const original = btnText.textContent;
        btnText.textContent = 'Copied!';
        setTimeout(() => { btnText.textContent = original; }, 1500);
    } catch {
        // Fallback for older browsers
        const textarea = document.createElement('textarea');
        textarea.value = text;
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
        showToast('Quote copied!');
    }
}

// ── History Management ──────────────────────────────────────
function addToHistory(quote) {
    // Avoid duplicates
    if (quoteHistory.some(q => q.id === quote.id)) return;

    quoteHistory.unshift(quote);

    // Keep only last 5
    if (quoteHistory.length > 5) quoteHistory.pop();

    renderHistory();
}

function renderHistory() {
    if (quoteHistory.length === 0) {
        historySection.style.display = 'none';
        return;
    }

    historySection.style.display = 'block';
    historyList.innerHTML = quoteHistory
        .map(q => `
            <div class="history-item">
                <blockquote class="quote-text">${escapeHtml(q.quote)}</blockquote>
                <cite class="quote-author">— ${escapeHtml(q.author)}</cite>
            </div>
        `)
        .join('');
}

// ── UI Helpers ──────────────────────────────────────────────
function setLoading(isLoading) {
    if (isLoading) {
        generateBtn.classList.add('loading');
        generateBtn.disabled = true;
        statusDot.className = 'status-dot loading';
        statusText.textContent = 'Fetching...';
    } else {
        generateBtn.classList.remove('loading');
        generateBtn.disabled = false;
    }
}

function setStatus(type, message) {
    statusDot.className = `status-dot ${type === 'error' ? 'error' : ''}`;
    statusText.textContent = message;

    if (type === 'success') {
        setTimeout(() => {
            statusDot.className = 'status-dot';
            statusText.textContent = 'Ready';
        }, 3000);
    }
}

function showToast(message) {
    // Remove existing toast
    const existing = document.querySelector('.toast');
    if (existing) existing.remove();

    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.textContent = message;
    document.body.appendChild(toast);

    requestAnimationFrame(() => {
        toast.classList.add('show');
    });

    setTimeout(() => {
        toast.classList.remove('show');
        setTimeout(() => toast.remove(), 300);
    }, 2500);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ── Health Check on Load ────────────────────────────────────
async function checkHealth() {
    try {
        const response = await fetch(`${API_BASE}/health`);
        if (response.ok) {
            setStatus('success', 'Connected');
            setTimeout(() => {
                statusDot.className = 'status-dot';
                statusText.textContent = 'Ready';
            }, 2000);
        } else {
            setStatus('error', 'API unhealthy');
        }
    } catch {
        setStatus('error', 'API offline');
    }
}

// ── Keyboard Shortcut ───────────────────────────────────────
document.addEventListener('keydown', (e) => {
    if (e.code === 'Space' && !e.target.matches('input, textarea')) {
        e.preventDefault();
        generateQuote();
    }
});

// ── Initialize ──────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', () => {
    checkHealth();
});
