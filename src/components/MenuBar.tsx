import '../styles/MenuBar.css'

const MENU_ITEMS = ['File', 'Edit', 'Options', 'Buffers', 'Tools', 'Help']

export default function MenuBar() {
  return (
    <div className="menu-bar">
      {MENU_ITEMS.map((item) => (
        <span key={item} className="menu-bar-item">
          {item}
        </span>
      ))}
    </div>
  )
}
