import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
function MyComponent({ displayText }: { displayText: string }) {
  return (
    <div>
      <p>{displayText}</p>
    </div>
  )
}

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>React App</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the React logos to learn more

        about these projects.

        Use "\" to escape special characters when searching.
      </p>
      <MyComponent 
        displayText="
        Hello, world!
        This is a test for multiline text in jsx attributes. And it should not add
        extra \n characters in between the text."
      />
    </>
  )
}

export default App
