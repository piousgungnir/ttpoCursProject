import {useState} from "react";
import {MathJax, MathJaxContext} from "better-react-mathjax";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";

const config = {
    loader: {load: ["[tex]/html"]},
    tex: {
        packages: {"[+]": ["html"]},
        inlineMath: [
            ["$", "$"],
            ["\\(", "\\)"]
        ],
        displayMath: [
            ["$$", "$$"],
            ["\\[", "\\]"]
        ]
    }
};

function App() {
    const [expressionResult, setExpressionResult] = useState("");
    const [expression, setExpression] = useState("");

    async function solveExpr() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setExpressionResult(await invoke("solve_expr", {expression: expression.slice(1, -1)}));
    }

    return (
        <MathJaxContext version={3} config={config}>
            <div className="container">
                <h1>Welcome to my tiny calculator!</h1>

                <div className="row">
                    <a href="https://youtu.be/u3e-CUE7ljk?t=102" target="_blank">
                        <img src="/naclz.png" className="logo" alt="logo"/>
                    </a>
                </div>

                <form
                    className="row"
                    onSubmit={(e) => {
                        e.preventDefault();
                        solveExpr();
                    }}
                >
                    <textarea
                        id="expression-input"
                        onChange={(e) => setExpression("$" + e.currentTarget.value + "$")}
                        placeholder="Enter a expression..."
                        rows={4} cols={50}
                    />
                    <button type="submit">Solve expression</button>
                </form>

                <div>
                    <p>Entered expression: </p>
                        <MathJax
                            inline
                            dynamic
                            hideUntilTypeset={"every"}
                        >
                            {expression}
                        </MathJax>

                </div>

                <div>
                    <p>Result(s): </p>
                        <MathJax
                            inline
                            dynamic
                            hideUntilTypeset={"every"}
                        >
                            {expressionResult}
                        </MathJax>
                </div>

            </div>

        </MathJaxContext>
    )
        ;
}

export default App;
