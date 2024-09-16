import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:convert';

void main() {
  runApp(MyApp());
}

// Make MyApp a StatefulWidget
class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  // Theme mode state variable
  ThemeMode _themeMode = ThemeMode.light;

  // Function to toggle theme mode
  void _toggleTheme() {
    setState(() {
      _themeMode =
          _themeMode == ThemeMode.light ? ThemeMode.dark : ThemeMode.light;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Collaborative Text Editor',
      theme: ThemeData.light(),
      darkTheme: ThemeData.dark(),
      themeMode: _themeMode,
      home: EditorPage(toggleTheme: _toggleTheme),
    );
  }
}

class EditorPage extends StatefulWidget {
  final VoidCallback toggleTheme;

  // Accept the toggleTheme function in the constructor
  EditorPage({required this.toggleTheme});

  @override
  _EditorPageState createState() => _EditorPageState();
}

class _EditorPageState extends State<EditorPage> {
  final TextEditingController _controller = TextEditingController();
  late WebSocketChannel channel;
  String _content = '';
  int _version = 0;
  bool _updatingTextField = false;

  @override
  void initState() {
    super.initState();
    connectToServer();
  }

  void connectToServer() {
    channel = WebSocketChannel.connect(Uri.parse('ws://localhost:8080'));

    channel.stream.listen((message) {
      final data = json.decode(message);
      setState(() {
        if (data['type'] == 'initial') {
          // Handle initial document state
          _content = data['content'];
          _version = data['version'];
          _updatingTextField = true;
          _controller.text = _content;
          _controller.selection = TextSelection.fromPosition(
              TextPosition(offset: _controller.text.length));
          _updatingTextField = false;
        } else if (data['type'] == 'edit') {
          // Handle incoming edits
          final edit = data['edit'];
          if (edit['version'] > _version) {
            if (edit['insert'] != null) {
              _content = _content.substring(0, edit['position']) +
                  edit['insert'] +
                  _content.substring(edit['position']);
            } else if (edit['delete'] != null) {
              _content = _content.substring(0, edit['position']) +
                  _content.substring(edit['position'] + edit['delete']);
            }
            _version = edit['version'];
            _updatingTextField = true;
            _controller.text = _content;
            _controller.selection = TextSelection.fromPosition(
                TextPosition(offset: _controller.text.length));
            _updatingTextField = false;
          } else {
            // Version mismatch, request full document state
            channel.sink.add(json.encode({'type': 'request_full_state'}));
          }
        } else if (data['type'] == 'full_state') {
          // Handle full document state update
          _content = data['content'];
          _version = data['version'];
          _updatingTextField = true;
          _controller.text = _content;
          _controller.selection = TextSelection.fromPosition(
              TextPosition(offset: _controller.text.length));
          _updatingTextField = false;
        }
      });
    });
  }

  void _handleEdit(String newValue) {
    if (_updatingTextField) {
      return;
    }

    final oldValue = _content;
    int start = 0;

    while (start < oldValue.length &&
        start < newValue.length &&
        oldValue[start] == newValue[start]) {
      start++;
    }

    int endOld = oldValue.length;
    int endNew = newValue.length;

    while (endOld > start &&
        endNew > start &&
        oldValue[endOld - 1] == newValue[endNew - 1]) {
      endOld--;
      endNew--;
    }

    if (start < endOld || start < endNew) {
      if (endOld - start > 0) {
        final deleteCount = endOld - start;
        final edit = {
          'position': start,
          'delete': deleteCount,
          'version': _version,
        };
        channel.sink.add(json.encode({
          'type': 'edit',
          'edit': edit,
        }));
        _version++; // Increment version after sending edit
      }
      if (endNew - start > 0) {
        final insert = newValue.substring(start, endNew);
        final edit = {
          'position': start,
          'insert': insert,
          'version': _version,
        };
        channel.sink.add(json.encode({
          'type': 'edit',
          'edit': edit,
        }));
        _version++; // Increment version after sending edit
      }
    }

    _content = newValue;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Collaborative Text Editor'),
        actions: [
          IconButton(
            icon: Icon(Icons.brightness_6),
            onPressed: widget.toggleTheme,
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: TextField(
          controller: _controller,
          maxLines: null,
          expands: true,
          onChanged: _handleEdit,
          decoration: InputDecoration(
            border: OutlineInputBorder(),
            hintText: 'Start typing...',
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    channel.sink.close();
    _controller.dispose();
    super.dispose();
  }
}
