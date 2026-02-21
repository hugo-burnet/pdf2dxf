import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, message } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";

import { motion, AnimatePresence } from "framer-motion";
import {
    FileText,
    Monitor,
    LogOut,
    CheckCircle2,
    ChevronRight,
    ArrowRight,
    Upload,
    X,
    Minus,
    Linkedin,
    Github,
    Globe,
    LayoutDashboard,
} from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

interface Conversion {
    id: string;
    name: string;
    size: string;
    time: string;
    status: "processing" | "completed" | "error";
    progress?: number;
    path?: string;
}

function App() {
    const [inputPath, setInputPath] = useState<string | null>(null);
    const [status, setStatus] = useState<"idle" | "converting" | "success" | "error">("idle");
    const [recentConversions, setRecentConversions] = useState<Conversion[]>([]);

    // Modals State
    const [showScaleModal, setShowScaleModal] = useState(false);
    const [showSuccessModal, setShowSuccessModal] = useState(false);

    // Scale State
    const [scaleNum, setScaleNum] = useState<string>("1");
    const [scaleDenom, setScaleDenom] = useState<string>("1");

    useEffect(() => {
        const unlisten = appWindow.onDragDropEvent((event) => {
            if (event.payload.type === "drop") {
                const paths = event.payload.paths;
                if (paths && paths.length > 0 && paths[0].toLowerCase().endsWith(".pdf")) {
                    setInputPath(paths[0]);
                    setStatus("idle");
                }
            }
        });

        return () => {
            unlisten.then(f => f());
        };
    }, []);

    const handleSelectFile = async () => {
        try {
            const selected = await open({
                multiple: false,
                filters: [{ name: "PDF Files", extensions: ["pdf"] }],
            });
            if (selected && typeof selected === "string") {
                setInputPath(selected);
                setStatus("idle");
            }
        } catch (err) {
            console.error(err);
        }
    };

    const handleConvert = async () => {
        if (!inputPath) return;

        setStatus("converting");
        const startTime = Date.now();
        const pdfName = inputPath.split("/").pop() || "unknown.pdf";
        const dxfName = pdfName.replace(/\.[^/.]+$/, "") + ".dxf";
        const newId = Date.now().toString();

        const num = parseFloat(scaleNum);
        const denom = parseFloat(scaleDenom);
        if (isNaN(num) || isNaN(denom) || num <= 0 || denom <= 0) {
            await message("Valeur d'échelle invalide.", { title: "Erreur", kind: "error" });
            setStatus("idle");
            return;
        }

        const scaleFactor = num / denom;

        // Add to history immediately as processing
        setRecentConversions(prev => [
            { id: newId, name: dxfName, size: "Processing...", time: "Just now", status: "processing", progress: 10 },
            ...prev
        ]);

        try {
            const conversionPromise = invoke<string>("convert_pdf", {
                inputPath,
                scaleFactor,
                unit: "mm"
            });

            const progressInterval = setInterval(() => {
                setRecentConversions(prev => prev.map(c =>
                    c.id === newId ? { ...c, progress: Math.min(90, (c.progress || 10) + 5) } : c
                ));
            }, 300);

            const outputPathStr = await conversionPromise;
            clearInterval(progressInterval);

            const elapsedTime = Date.now() - startTime;
            if (elapsedTime < 3000) {
                await new Promise(resolve => setTimeout(resolve, 3000 - elapsedTime));
            }

            setStatus("success");
            setRecentConversions(prev => prev.map(c =>
                c.id === newId ? {
                    ...c,
                    status: "completed",
                    size: "Converted",
                    time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
                    progress: 100,
                    path: outputPathStr
                } : c
            ));

            setShowSuccessModal(true);
        } catch (err: any) {
            setStatus("idle");
            setRecentConversions(prev => prev.filter(c => c.id !== newId));
            await message(typeof err === "string" ? err : err.message || "La conversion a échoué.", { title: "Erreur de conversion", kind: "error" });
        }
    };

    const handleHistoryClick = async (conv: Conversion) => {
        if (conv.path) {
            try {
                await invoke("open_dxf", { path: conv.path });
            } catch (err: any) {
                console.error(err);
                await message(typeof err === "string" ? err : err.message || "Failed to open the file.", { title: "Error", kind: "error" });
            }
        }
    };

    return (
        <div className="app-container">
            {/* Sidebar History */}
            <aside className="sidebar">
                <div className="sidebar-header">
                    <h3>History</h3>
                </div>
                <div className="sidebar-content">
                    <AnimatePresence>
                        {recentConversions.map((conv) => (
                            <motion.div
                                key={conv.id}
                                layout
                                initial={{ opacity: 0, scale: 0.9, y: 10 }}
                                animate={{ opacity: 1, scale: 1, y: 0 }}
                                className="history-item animate-once"
                                onClick={() => handleHistoryClick(conv)}
                            >
                                <div className="history-icon">
                                    <FileText size={18} />
                                </div>
                                <div className="history-info">
                                    <div className="history-name">{conv.name}</div>
                                    <div className="history-meta">{conv.status === 'processing' ? `${conv.progress}%` : conv.time}</div>
                                </div>
                                {conv.status === 'completed' && <CheckCircle2 size={14} style={{ color: '#15803d' }} />}
                            </motion.div>
                        ))}
                    </AnimatePresence>
                    {recentConversions.length === 0 && (
                        <div style={{ padding: "20px", textAlign: "center", color: "var(--text-secondary)", fontSize: "12px" }}>
                            No history yet.
                        </div>
                    )}
                </div>

                <div className="sidebar-footer" style={{ padding: "16px", display: "flex", gap: "16px", justifyContent: "center", borderTop: "1px solid var(--border-color)", marginTop: "auto", background: "var(--bg-secondary)" }}>
                    <motion.div whileHover={{ scale: 1.1, color: "var(--accent-blue)" }} style={{ cursor: "pointer", color: "var(--text-secondary)" }} onClick={() => openUrl("https://www.linkedin.com/in/hugo-burnet-a11323309/")} title="LinkedIn">
                        <Linkedin size={20} />
                    </motion.div>
                    <motion.div whileHover={{ scale: 1.1, color: "var(--accent-blue)" }} style={{ cursor: "pointer", color: "var(--text-secondary)" }} onClick={() => openUrl("https://hugo-burnet.github.io/cv-online/")} title="Portfolio">
                        <Globe size={20} />
                    </motion.div>
                    <motion.div whileHover={{ scale: 1.1, color: "var(--accent-blue)" }} style={{ cursor: "pointer", color: "var(--text-secondary)" }} onClick={() => openUrl("https://hugo-burnet.github.io/CalipiCAD/")} title="CalipiCAD">
                        <LayoutDashboard size={20} />
                    </motion.div>
                    <motion.div whileHover={{ scale: 1.1, color: "var(--accent-blue)" }} style={{ cursor: "pointer", color: "var(--text-secondary)" }} onClick={() => openUrl("https://github.com/hugo-burnet")} title="GitHub">
                        <Github size={20} />
                    </motion.div>
                </div>
            </aside>

            {/* Main Content */}
            <main className="main-content">
                <div
                    className="scroll-content"
                    onPointerDown={(e) => {
                        // Allow dragging from the top/side padding of the scroll area
                        if (e.target === e.currentTarget) {
                            appWindow.startDragging().catch(console.error);
                        }
                    }}
                >
                    <header
                        className="header"
                        data-tauri-drag-region
                        onPointerDown={(e) => {
                            if (e.target === e.currentTarget) {
                                appWindow.startDragging().catch(console.error);
                            }
                        }}
                    >
                        <div className="header-title" style={{ pointerEvents: 'none' }}>
                            <div style={{ display: "flex", alignItems: "center", gap: "8px", fontSize: "12px", color: "var(--text-secondary)", marginBottom: "8px" }}>
                                PDF2DXF <ChevronRight size={12} /> <span style={{ color: "var(--text-primary)", fontWeight: 600 }}>CONVERTER</span>
                            </div>
                            <h1>PDF to DXF <span>Converter</span></h1>
                            <p>High-fidelity vector extraction for architectural plans.</p>
                        </div>

                        <div className="window-controls" style={{ pointerEvents: 'auto' }}>
                            <button className="control-btn minimize" onClick={() => appWindow.minimize()}>
                                <Minus size={16} />
                            </button>
                            <button className="control-btn close" onClick={() => appWindow.close()}>
                                <X size={16} />
                            </button>
                        </div>
                    </header>

                    {/* Dynamic Conversion Area */}
                    <motion.div
                        className="dropzone-container"
                        whileHover={status === 'idle' ? { scale: 1.005 } : {}}
                        onClick={status === 'idle' ? handleSelectFile : undefined}
                    >
                        <div className="corner top-left"></div>
                        <div className="corner top-right"></div>
                        <div className="corner bottom-left"></div>
                        <div className="corner bottom-right"></div>

                        <AnimatePresence mode="wait">
                            {status === 'converting' ? (
                                <motion.div
                                    key="converting"
                                    initial={{ opacity: 0, scale: 0.95 }}
                                    animate={{ opacity: 1, scale: 1 }}
                                    exit={{ opacity: 0, scale: 1.05 }}
                                    style={{ width: '100%', maxWidth: '400px', textAlign: 'center' }}
                                >
                                    <div className="loader-container" style={{ marginBottom: '24px' }}>
                                        <svg className="cad-loader" viewBox="0 0 50 50">
                                            <circle className="cad-loader-track" cx="25" cy="25" r="20"></circle>
                                            <circle className="cad-loader-dash" cx="25" cy="25" r="20"></circle>
                                        </svg>
                                    </div>
                                    <h2 style={{ marginBottom: '16px' }}>Converting {inputPath?.split('/').pop()}</h2>
                                    <div className="progress-bar" style={{ height: '8px', background: '#f1f5f9', borderRadius: '4px', overflow: 'hidden' }}>
                                        <motion.div
                                            className="progress-fill"
                                            initial={{ width: 0 }}
                                            animate={{ width: `${recentConversions[0]?.progress || 0}%` }}
                                            style={{ height: '100%', background: 'var(--accent-blue)' }}
                                        />
                                    </div>
                                    <p style={{ marginTop: '12px', color: 'var(--text-secondary)', fontSize: '14px' }}>
                                        {Math.round(recentConversions[0]?.progress || 0)}% completed
                                    </p>
                                </motion.div>
                            ) : (
                                <motion.div
                                    key="idle"
                                    initial={{ opacity: 0 }}
                                    animate={{ opacity: 1 }}
                                    exit={{ opacity: 0 }}
                                    style={{ textAlign: 'center' }}
                                >
                                    <div className="dropzone-cad-container">
                                        <div className="cad-crosshair-v"></div>
                                        <div className="cad-crosshair-h"></div>
                                        <div className="dropzone-icon">
                                            <Upload size={32} strokeWidth={1.5} />
                                        </div>
                                        <div className="dropzone-text">
                                            <h2>{inputPath ? inputPath.split("/").pop() : "DROP PDF HERE"}</h2>
                                            <p>DRAG & DROP OR CLICK TO BROWSE</p>
                                        </div>
                                    </div>
                                </motion.div>
                            )}
                        </AnimatePresence>
                    </motion.div>

                    {/* Config Selection */}
                    <div className="config-grid" style={{ opacity: status === 'converting' ? 0.5 : 1, pointerEvents: status === 'converting' ? 'none' : 'auto' }}>
                        <div className="config-card" onClick={() => setShowScaleModal(true)}>
                            <div className="config-info">
                                <label>Échelle Globale</label>
                                <p>{scaleNum || 1} / {scaleDenom || 1}</p>
                            </div>
                            <Monitor size={18} className="config-icon" />
                        </div>
                        <div className="config-card">
                            <div className="config-info">
                                <label>Output Format</label>
                                <p>AutoCAD DXF R12</p>
                            </div>
                            <LogOut size={18} className="config-icon" style={{ transform: "rotate(90deg)" }} />
                        </div>
                    </div>
                </div>

                {/* Action Bar */}
                <div className="footer-bar">
                    <div className="action-buttons" style={{ marginLeft: "auto" }}>
                        <button className="btn-ghost" onClick={() => setRecentConversions([])}>Clear History</button>
                        <button
                            className="btn-primary"
                            onClick={handleConvert}
                            disabled={!inputPath || status === "converting"}
                        >
                            {status === "converting" ? "Processing..." : "Start Conversion"}
                            <ArrowRight size={18} />
                        </button>
                    </div>
                </div>
            </main>

            {/* Scale Modal */}
            <AnimatePresence>
                {showScaleModal && (
                    <motion.div
                        className="modal-overlay"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        onClick={() => setShowScaleModal(false)}
                    >
                        <motion.div
                            className="modal-content"
                            initial={{ scale: 0.9, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            exit={{ scale: 0.9, opacity: 0 }}
                            onClick={(e) => e.stopPropagation()}
                        >
                            <div className="modal-header">
                                <h2>Scale Settings</h2>
                                <p>Define the scale multiplier for the output DXF.</p>
                            </div>

                            <div className="input-group">
                                <label>Ratio d'échelle</label>
                                <div style={{ display: "flex", alignItems: "center", gap: "8px", background: "var(--bg-secondary)", border: "1px solid var(--border-color)", borderRadius: "var(--radius-md)", padding: "4px 8px" }}>
                                    <input
                                        type="number"
                                        value={scaleNum}
                                        onChange={(e) => setScaleNum(e.target.value)}
                                        step="0.001"
                                        style={{ flex: 1, minWidth: "0", display: "block", border: "none", padding: "6px", background: "var(--bg-primary)", borderRadius: "4px", outline: "none", color: "var(--text-primary)", textAlign: "center", appearance: "textfield" }}
                                    />
                                    <span style={{ fontWeight: 600, color: "var(--text-secondary)", flexShrink: 0 }}>/</span>
                                    <input
                                        type="number"
                                        value={scaleDenom}
                                        onChange={(e) => setScaleDenom(e.target.value)}
                                        step="0.001"
                                        style={{ flex: 1, minWidth: "0", display: "block", border: "none", padding: "6px", background: "var(--bg-primary)", borderRadius: "4px", outline: "none", color: "var(--text-primary)", textAlign: "center", appearance: "textfield" }}
                                    />
                                </div>
                            </div>

                            <button className="btn-primary" style={{ width: "100%", justifyContent: "center" }} onClick={() => setShowScaleModal(false)}>
                                Apply Settings
                            </button>
                        </motion.div>
                    </motion.div>
                )}
            </AnimatePresence>

            {/* Success Modal */}
            <AnimatePresence>
                {showSuccessModal && (
                    <motion.div
                        className="modal-overlay"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        onClick={() => {
                            setShowSuccessModal(false);
                            setInputPath(null);
                            setStatus("idle");
                        }}
                    >
                        <motion.div
                            className="modal-content"
                            style={{ textAlign: "center", alignItems: "center" }}
                            initial={{ scale: 0.9, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            exit={{ scale: 0.9, opacity: 0 }}
                            onClick={(e) => e.stopPropagation()}
                        >
                            <div style={{
                                width: "64px",
                                height: "64px",
                                background: "#f0fdf4",
                                borderRadius: "50%",
                                display: "flex",
                                alignItems: "center",
                                justifyContent: "center",
                                color: "#15803d",
                                marginBottom: "16px"
                            }}>
                                <CheckCircle2 size={32} />
                            </div>
                            <div className="modal-header">
                                <h2>Conversion Complete!</h2>
                                <p>Your DXF file has been generated successfully and is ready for use in CAD software.</p>
                            </div>
                            <button className="btn-primary" style={{ width: "100%", justifyContent: "center", marginTop: "8px" }} onClick={() => {
                                setShowSuccessModal(false);
                                setInputPath(null);
                                setStatus("idle");
                            }}>
                                Got it
                            </button>
                        </motion.div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}

export default App;
