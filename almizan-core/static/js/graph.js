// Graph Visualization Logic - Advanced Features

// Global state for graph interactions
const graphState = {
    activeFilters: new Set(['verse', 'hadith', 'ruling']),
    searchTerm: '',
    selectedNode: null
};

document.addEventListener('DOMContentLoaded', async () => {
    const container = document.getElementById('graph-container');
    const width = container.clientWidth || 800;
    const height = container.clientHeight || 600;

    // Create SVG with zoom container
    const svg = d3.select("#graph-container")
        .append("svg")
        .attr("width", width)
        .attr("height", height);

    // Create a group for zoom/pan transformations
    const g = svg.append("g");

    // Set up zoom behavior
    const zoom = d3.zoom()
        .scaleExtent([0.1, 4])
        .on("zoom", (event) => {
            g.attr("transform", event.transform);
        });

    svg.call(zoom);

    // Expose zoom controls globally
    window.graphZoom = {
        zoomIn: () => svg.transition().call(zoom.scaleBy, 1.3),
        zoomOut: () => svg.transition().call(zoom.scaleBy, 0.7),
        reset: () => svg.transition().call(zoom.transform, d3.zoomIdentity)
    };

    // Add loading indicator (to the zoom group)
    const loadingText = g.append("text")
        .attr("x", width / 2)
        .attr("y", height / 2)
        .attr("text-anchor", "middle")
        .attr("fill", "#D4AF37")
        .attr("font-size", "18px")
        .text("Loading Knowledge Graph...");

    // Fetch real data from API
    let graphData;
    try {
        const response = await fetch('/api/v1/graph');
        graphData = await response.json();
        
        // Transform API response (nodes are wrapped in { data: {} })
        const nodes = graphData.nodes.map(n => ({
            id: n.data.id,
            label: n.data.label,
            type: n.data.type,
            tier: n.data.type === 'verse' ? 'thabit' : 'context'
        }));
        
        const edges = graphData.edges.map(e => ({
            source: e.data.source,
            target: e.data.target
        }));

        // Remove loading text
        loadingText.remove();
        
        renderGraph(nodes, edges);
    } catch (error) {
        loadingText.text("Error loading graph. Using sample data.");
        console.error('Graph API error:', error);
        
        // Fallback to sample data
        setTimeout(() => {
            loadingText.remove();
            renderGraph([
                { id: "verse:1_1", label: "Al-Fatiha 1:1", type: "verse", tier: "thabit" },
                { id: "root:b-s-m", label: "ب-س-م", type: "root", tier: "context" },
                { id: "root:r-h-m", label: "ر-ح-م", type: "root", tier: "context" }
            ], [
                { source: "verse:1_1", target: "root:b-s-m" },
                { source: "verse:1_1", target: "root:r-h-m" }
            ]);
        }, 1000);
    }

    function renderGraph(nodes, links) {
        // Create simulation
        const simulation = d3.forceSimulation(nodes)
            .force("link", d3.forceLink(links).id(d => d.id).distance(80))
            .force("charge", d3.forceManyBody().strength(-200))
            .force("center", d3.forceCenter(width / 2, height / 2))
            .force("collision", d3.forceCollide().radius(30));

        // Draw links (add to zoom group)
        const link = g.append("g")
            .attr("class", "links")
            .selectAll("line")
            .data(links)
            .enter().append("line")
            .attr("stroke", "#888")
            .attr("stroke-width", 2)
            .attr("stroke-opacity", 0.5);

        // Draw nodes (add to zoom group)
        const node = g.append("g")
            .attr("class", "nodes")
            .selectAll("g")
            .data(nodes)
            .enter().append("g")
            .attr("class", "node")
            .style("cursor", "pointer")
            .call(d3.drag()
                .on("start", dragstarted)
                .on("drag", dragged)
                .on("end", dragended))
            .on("mouseenter", function(event, d) {
                d3.select(this).select("circle")
                    .transition().duration(200)
                    .attr("r", d.tier === 'thabit' ? 16 : 12)
                    .attr("stroke-width", 3);
                
                // Highlight connected nodes
                const connectedIds = new Set();
                links.forEach(l => {
                    if (l.source.id === d.id) connectedIds.add(l.target.id);
                    if (l.target.id === d.id) connectedIds.add(l.source.id);
                });
                
                node.style("opacity", n => connectedIds.has(n.id) || n.id === d.id ? 1 : 0.3);
                link.style("opacity", l => 
                    (l.source.id === d.id || l.target.id === d.id) ? 1 : 0.1
                );
            })
            .on("mouseleave", function(event, d) {
                d3.select(this).select("circle")
                    .transition().duration(200)
                    .attr("r", d.tier === 'thabit' ? 12 : 8)
                    .attr("stroke-width", 2);
                
                node.style("opacity", 1);
                link.style("opacity", 0.6);
            })
            .on("click", function(event, d) {
                graphState.selectedNode = d;
                
                // Pulse animation on click
                d3.select(this).select("circle")
                    .transition()
                    .duration(300)
                    .attr("r", d.tier === 'thabit' ? 18 : 14)
                    .transition()
                    .duration(300)
                    .attr("r", d.tier === 'thabit' ? 12 : 8);
                
                showNodeDetails(d, links);
            });

        // Node circles with colors by type
        node.append("circle")
            .attr("r", d => d.tier === 'thabit' ? 12 : 8)
            .attr("fill", d => {
                if (d.type === 'verse') return '#D4AF37';  // Gold for Quran
                if (d.type === 'root') return '#00CED1';   // Cyan for Roots
                if (d.type === 'hadith') return '#C0C0C0'; // Silver
                if (d.type === 'ruling') return '#DC143C'; // Crimson
                return '#4682B4';  // Blue default
            })
            .attr("stroke", "#fff")
            .attr("stroke-width", 2);

        // Labels
        node.append("text")
            .text(d => d.label)
            .attr("x", 15)
            .attr("y", 4)
            .attr("fill", "#fff")
            .attr("font-size", "14px")
            .attr("font-weight", "500")
            .style("text-shadow", "0 0 4px rgba(0,0,0,0.8)");

        // Tick handler
        simulation.on("tick", () => {
            link
                .attr("x1", d => d.source.x)
                .attr("y1", d => d.source.y)
                .attr("x2", d => d.target.x)
                .attr("y2", d => d.target.y);

            node.attr("transform", d => `translate(${d.x},${d.y})`);
        });

        // Search functionality
        window.searchGraph = function(term) {
            graphState.searchTerm = term.toLowerCase();
            
            if (!term) {
                node.style("opacity", 1);
                node.selectAll("circle").attr("stroke", "#fff").attr("stroke-width", 2);
                return;
            }
            
            const matches = nodes.filter(n => 
                n.id.toLowerCase().includes(term) || n.label.toLowerCase().includes(term)
            );
            
            if (matches.length > 0) {
                // Highlight matches
                node.style("opacity", n => matches.find(m => m.id === n.id) ? 1 : 0.2);
                node.selectAll("circle")
                    .attr("stroke", d => matches.find(m => m.id === d.id) ? "#D4AF37" : "#fff")
                    .attr("stroke-width", d => matches.find(m => m.id === d.id) ? 4 : 2);
                
                // Center on first match
                const firstMatch = matches[0];
                const transform = d3.zoomIdentity
                    .translate(width / 2, height / 2)
                    .scale(1.5)
                    .translate(-firstMatch.x, -firstMatch.y);
                svg.transition().duration(750).call(zoom.transform, transform);
            } else {
                node.style("opacity", 0.2);
            }
        };

        // Filter functionality
        window.toggleFilter = function(type, isChecked) {
            if (isChecked) {
                graphState.activeFilters.add(type);
            } else {
                graphState.activeFilters.delete(type);
            }
            
            node.transition().duration(300)
                .style("opacity", d => graphState.activeFilters.has(d.type) ? 1 : 0)
                .style("pointer-events", d => graphState.activeFilters.has(d.type) ? "all" : "none");
            
            link.transition().duration(300)
                .style("opacity", l => {
                    const sourceVisible = graphState.activeFilters.has(l.source.type);
                    const targetVisible = graphState.activeFilters.has(l.target.type);
                    return (sourceVisible && targetVisible) ? 0.6 : 0;
                });
        };

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
    }

    // Node details panel
    function showNodeDetails(node, links) {
        const panel = document.getElementById('node-details');
        const connections = links.filter(l => l.source.id === node.id || l.target.id === node.id);
        
        document.getElementById('detail-id').textContent = node.id;
        document.getElementById('detail-label').textContent = node.label;
        document.getElementById('detail-type').textContent = node.type.toUpperCase();
        document.getElementById('detail-connections').textContent = connections.length;
        
        panel.classList.add('active');
    }

    window.closeNodeDetails = function() {
        document.getElementById('node-details').classList.remove('active');
    };
});
