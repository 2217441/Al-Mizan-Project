// Graph Visualization Logic

document.addEventListener('DOMContentLoaded', () => {
    const width = document.getElementById('graph-container').clientWidth;
    const height = document.getElementById('graph-container').clientHeight;

    const svg = d3.select("#graph-container")
        .append("svg")
        .attr("width", width)
        .attr("height", height);

    // Initial Mock Data (Synchronized with Phase 1 Mock)
    const initialData = {
        nodes: [
            { id: "verse:2_255", label: "Ayatul Kursi", type: "quran_verse", tier: "thabit" },
            { id: "hadith:bukhari_1", label: "Hadith: Intentions", type: "hadith", tier: "thabit" },
            { id: "ruling:wudu_intent", label: "Wudu Niyyah", type: "fiqh_ruling", tier: "zanni" },
            { id: "scholar:shafii", label: "Imam Shafi'i", type: "scholar", tier: "context" }
        ],
        links: [
            { source: "hadith:bukhari_1", target: "verse:2_255", type: "EXPLAINS" },
            { source: "ruling:wudu_intent", target: "hadith:bukhari_1", type: "DERIVED_FROM" },
            { source: "scholar:shafii", target: "ruling:wudu_intent", type: "AUTHORED" }
        ]
    };

    const simulation = d3.forceSimulation(initialData.nodes)
        .force("link", d3.forceLink(initialData.links).id(d => d.id).distance(150))
        .force("charge", d3.forceManyBody().strength(-300))
        .force("center", d3.forceCenter(width / 2, height / 2));

    // Links
    const link = svg.append("g")
        .selectAll("line")
        .data(initialData.links)
        .enter().append("line")
        .attr("class", "link")
        .attr("stroke", "#555")
        .attr("stroke-width", 1.5);

    // Node Groups
    const node = svg.append("g")
        .selectAll("g")
        .data(initialData.nodes)
        .enter().append("g")
        .attr("class", "node")
        .call(d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended));

    // Circles
    node.append("circle")
        .attr("r", d => d.tier === 'thabit' ? 15 : 10)
        .attr("fill", d => {
            if (d.type === 'quran_verse') return '#D4AF37'; // Gold
            if (d.type === 'hadith') return '#C0C0C0'; // Silver
            if (d.type === 'fiqh_ruling') return '#DC143C'; // Crimson
            return '#4682B4'; // Blue
        })
        .attr("stroke", "#fff");

    // Labels
    node.append("text")
        .text(d => d.label)
        .attr("x", 12)
        .attr("y", 4);

    // Simulation Tick
    simulation.on("tick", () => {
        link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);

        node
            .attr("transform", d => `translate(${d.x},${d.y})`);
    });

    function dragstarted(event, d) {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
    }

    function dragged(event, d) {
        d.fx = event.x;
        d.fy = event.y;
    }

    function dragended(event, d) {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
    }
});
