use anyhow::{bail, Result};

use crate::ast::Value;

pub fn get_color_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => {
            Ok(match &**iden {
                // Tailwind colors
                "transparent" => String::from("transparent"),
                "inherit" => String::from("inherit"),
                "current" => String::from("currentColor"),
                "black" => String::from("#000000"),
                "white" => String::from("#ffffff"),

                // Slate
                "slate-50" => String::from("#f8fafc"),
                "slate-100" => String::from("#f1f5f9"),
                "slate-200" => String::from("#e2e8f0"),
                "slate-300" => String::from("#cbd5e1"),
                "slate-400" => String::from("#94a3b8"),
                "slate-500" => String::from("#64748b"),
                "slate-600" => String::from("#475569"),
                "slate-700" => String::from("#334155"),
                "slate-800" => String::from("#1e293b"),
                "slate-900" => String::from("#0f172a"),
                // Gray
                "gray-50" => String::from("#f9fafb"),
                "gray-100" => String::from("#f3f4f6"),
                "gray-200" => String::from("#e5e7eb"),
                "gray-300" => String::from("#d1d5db"),
                "gray-400" => String::from("#9ca3af"),
                "gray-500" => String::from("#6b7280"),
                "gray-600" => String::from("#4b5563"),
                "gray-700" => String::from("#374151"),
                "gray-800" => String::from("#1f2937"),
                "gray-900" => String::from("#111827"),
                // Zinc
                "zinc-50" => String::from("#fafafa"),
                "zinc-100" => String::from("#f4f4f5"),
                "zinc-200" => String::from("#e4e4e7"),
                "zinc-300" => String::from("#d4d4d8"),
                "zinc-400" => String::from("#a1a1aa"),
                "zinc-500" => String::from("#71717a"),
                "zinc-600" => String::from("#52525b"),
                "zinc-700" => String::from("#3f3f46"),
                "zinc-800" => String::from("#27272a"),
                "zinc-900" => String::from("#18181b"),
                // Neutral
                "neutral-50" => String::from("#fafafa"),
                "neutral-100" => String::from("#f5f5f5"),
                "neutral-200" => String::from("#e5e5e5"),
                "neutral-300" => String::from("#d4d4d4"),
                "neutral-400" => String::from("#a3a3a3"),
                "neutral-500" => String::from("#737373"),
                "neutral-600" => String::from("#525252"),
                "neutral-700" => String::from("#404040"),
                "neutral-800" => String::from("#262626"),
                "neutral-900" => String::from("#171717"),
                // Stone
                "stone-50" => String::from("#fafaf9"),
                "stone-100" => String::from("#f5f5f4"),
                "stone-200" => String::from("#e7e5e4"),
                "stone-300" => String::from("#d6d3d1"),
                "stone-400" => String::from("#a8a29e"),
                "stone-500" => String::from("#78716c"),
                "stone-600" => String::from("#57534e"),
                "stone-700" => String::from("#44403c"),
                "stone-800" => String::from("#292524"),
                "stone-900" => String::from("#1c1917"),
                // Red
                "red-50" => String::from("#fef2f2"),
                "red-100" => String::from("#fee2e2"),
                "red-200" => String::from("#fecaca"),
                "red-300" => String::from("#fca5a5"),
                "red-400" => String::from("#f87171"),
                "red-500" => String::from("#ef4444"),
                "red-600" => String::from("#dc2626"),
                "red-700" => String::from("#b91c1c"),
                "red-800" => String::from("#991b1b"),
                "red-900" => String::from("#7f1d1d"),
                // Orange
                "orange-50" => String::from("#fff7ed"),
                "orange-100" => String::from("#ffedd5"),
                "orange-200" => String::from("#fed7aa"),
                "orange-300" => String::from("#fdba74"),
                "orange-400" => String::from("#fb923c"),
                "orange-500" => String::from("#f97316"),
                "orange-600" => String::from("#ea580c"),
                "orange-700" => String::from("#c2410c"),
                "orange-800" => String::from("#9a3412"),
                "orange-900" => String::from("#7c2d12"),
                // Amber
                "amber-50" => String::from("#fffbeb"),
                "amber-100" => String::from("#fef3c7"),
                "amber-200" => String::from("#fde68a"),
                "amber-300" => String::from("#fcd34d"),
                "amber-400" => String::from("#fbbf24"),
                "amber-500" => String::from("#f59e0b"),
                "amber-600" => String::from("#d97706"),
                "amber-700" => String::from("#b45309"),
                "amber-800" => String::from("#92400e"),
                "amber-900" => String::from("#78350f"),
                // Yellow
                "yellow-50" => String::from("#fefce8"),
                "yellow-100" => String::from("#fef9c3"),
                "yellow-200" => String::from("#fef08a"),
                "yellow-300" => String::from("#fde047"),
                "yellow-400" => String::from("#facc15"),
                "yellow-500" => String::from("#eab308"),
                "yellow-600" => String::from("#ca8a04"),
                "yellow-700" => String::from("#a16207"),
                "yellow-800" => String::from("#854d0e"),
                "yellow-900" => String::from("#713f12"),
                // Lime
                "lime-50" => String::from("#f7fee7"),
                "lime-100" => String::from("#ecfccb"),
                "lime-200" => String::from("#d9f99d"),
                "lime-300" => String::from("#bef264"),
                "lime-400" => String::from("#a3e635"),
                "lime-500" => String::from("#84cc16"),
                "lime-600" => String::from("#65a30d"),
                "lime-700" => String::from("#4d7c0f"),
                "lime-800" => String::from("#3f6212"),
                "lime-900" => String::from("#365314"),
                // Green
                "green-50" => String::from("#f0fdf4"),
                "green-100" => String::from("#dcfce7"),
                "green-200" => String::from("#bbf7d0"),
                "green-300" => String::from("#86efac"),
                "green-400" => String::from("#4ade80"),
                "green-500" => String::from("#22c55e"),
                "green-600" => String::from("#16a34a"),
                "green-700" => String::from("#15803d"),
                "green-800" => String::from("#166534"),
                "green-900" => String::from("#14532d"),
                // Emerald
                "emerald-50" => String::from("#ecfdf5"),
                "emerald-100" => String::from("#d1fae5"),
                "emerald-200" => String::from("#a7f3d0"),
                "emerald-300" => String::from("#6ee7b7"),
                "emerald-400" => String::from("#34d399"),
                "emerald-500" => String::from("#10b981"),
                "emerald-600" => String::from("#059669"),
                "emerald-700" => String::from("#047857"),
                "emerald-800" => String::from("#065f46"),
                "emerald-900" => String::from("#064e3b"),
                // Teal
                "teal-50" => String::from("#f0fdfa"),
                "teal-100" => String::from("#ccfbf1"),
                "teal-200" => String::from("#99f6e4"),
                "teal-300" => String::from("#5eead4"),
                "teal-400" => String::from("#2dd4bf"),
                "teal-500" => String::from("#14b8a6"),
                "teal-600" => String::from("#0d9488"),
                "teal-700" => String::from("#0f766e"),
                "teal-800" => String::from("#115e59"),
                "teal-900" => String::from("#134e4a"),
                // Cyan
                "cyan-50" => String::from("#ecfeff"),
                "cyan-100" => String::from("#cffafe"),
                "cyan-200" => String::from("#a5f3fc"),
                "cyan-300" => String::from("#67e8f9"),
                "cyan-400" => String::from("#22d3ee"),
                "cyan-500" => String::from("#06b6d4"),
                "cyan-600" => String::from("#0891b2"),
                "cyan-700" => String::from("#0e7490"),
                "cyan-800" => String::from("#155e75"),
                "cyan-900" => String::from("#164e63"),
                // Sky
                "sky-50" => String::from("#f0f9ff"),
                "sky-100" => String::from("#e0f2fe"),
                "sky-200" => String::from("#bae6fd"),
                "sky-300" => String::from("#7dd3fc"),
                "sky-400" => String::from("#38bdf8"),
                "sky-500" => String::from("#0ea5e9"),
                "sky-600" => String::from("#0284c7"),
                "sky-700" => String::from("#0369a1"),
                "sky-800" => String::from("#075985"),
                "sky-900" => String::from("#0c4a6e"),
                // Blue
                "blue-50" => String::from("#eff6ff"),
                "blue-100" => String::from("#dbeafe"),
                "blue-200" => String::from("#bfdbfe"),
                "blue-300" => String::from("#93c5fd"),
                "blue-400" => String::from("#60a5fa"),
                "blue-500" => String::from("#3b82f6"),
                "blue-600" => String::from("#2563eb"),
                "blue-700" => String::from("#1d4ed8"),
                "blue-800" => String::from("#1e40af"),
                "blue-900" => String::from("#1e3a8a"),
                // Indigo
                "indigo-50" => String::from("#eef2ff"),
                "indigo-100" => String::from("#e0e7ff"),
                "indigo-200" => String::from("#c7d2fe"),
                "indigo-300" => String::from("#a5b4fc"),
                "indigo-400" => String::from("#818cf8"),
                "indigo-500" => String::from("#6366f1"),
                "indigo-600" => String::from("#4f46e5"),
                "indigo-700" => String::from("#4338ca"),
                "indigo-800" => String::from("#3730a3"),
                "indigo-900" => String::from("#312e81"),
                // Violet
                "violet-50" => String::from("#f5f3ff"),
                "violet-100" => String::from("#ede9fe"),
                "violet-200" => String::from("#ddd6fe"),
                "violet-300" => String::from("#c4b5fd"),
                "violet-400" => String::from("#a78bfa"),
                "violet-500" => String::from("#8b5cf6"),
                "violet-600" => String::from("#7c3aed"),
                "violet-700" => String::from("#6d28d9"),
                "violet-800" => String::from("#5b21b6"),
                "violet-900" => String::from("#4c1d95"),
                // Purple
                "purple-50" => String::from("#faf5ff"),
                "purple-100" => String::from("#f3e8ff"),
                "purple-200" => String::from("#e9d5ff"),
                "purple-300" => String::from("#d8b4fe"),
                "purple-400" => String::from("#c084fc"),
                "purple-500" => String::from("#a855f7"),
                "purple-600" => String::from("#9333ea"),
                "purple-700" => String::from("#7e22ce"),
                "purple-800" => String::from("#6b21a8"),
                "purple-900" => String::from("#581c87"),
                // Fuchsia
                "fuchsia-50" => String::from("#fdf4ff"),
                "fuchsia-100" => String::from("#fae8ff"),
                "fuchsia-200" => String::from("#f5d0fe"),
                "fuchsia-300" => String::from("#f0abfc"),
                "fuchsia-400" => String::from("#e879f9"),
                "fuchsia-500" => String::from("#d946ef"),
                "fuchsia-600" => String::from("#c026d3"),
                "fuchsia-700" => String::from("#a21caf"),
                "fuchsia-800" => String::from("#86198f"),
                "fuchsia-900" => String::from("#701a75"),
                // Pink
                "pink-50" => String::from("#fdf2f8"),
                "pink-100" => String::from("#fce7f3"),
                "pink-200" => String::from("#fbcfe8"),
                "pink-300" => String::from("#f9a8d4"),
                "pink-400" => String::from("#f472b6"),
                "pink-500" => String::from("#ec4899"),
                "pink-600" => String::from("#db2777"),
                "pink-700" => String::from("#be185d"),
                "pink-800" => String::from("#9d174d"),
                "pink-900" => String::from("#831843"),
                // Rose
                "rose-50" => String::from("#fff1f2"),
                "rose-100" => String::from("#ffe4e6"),
                "rose-200" => String::from("#fecdd3"),
                "rose-300" => String::from("#fda4af"),
                "rose-400" => String::from("#fb7185"),
                "rose-500" => String::from("#f43f5e"),
                "rose-600" => String::from("#e11d48"),
                "rose-700" => String::from("#be123c"),
                "rose-800" => String::from("#9f1239"),
                "rose-900" => String::from("#881337"),
                _ => bail!("Unknown value: {}", iden),
            })
        }
    }
}
