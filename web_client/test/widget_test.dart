import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:web_client/main.dart'; // Replace with your actual package name

void main() {
  testWidgets('Collaborative Text Editor app test',
      (WidgetTester tester) async {
    // Build the app and trigger a frame.
    await tester.pumpWidget(MyApp()); // Removed 'const' keyword

    // Verify that the app displays the correct title.
    expect(find.text('Collaborative Text Editor'), findsOneWidget);

    // Verify that the theme toggle button is present.
    expect(find.byIcon(Icons.brightness_6), findsOneWidget);

    // Tap the theme toggle button to switch themes.
    await tester.tap(find.byIcon(Icons.brightness_6));
    await tester.pumpAndSettle();

    // Verify that the theme has changed (optional; requires additional setup).

    // Verify that the TextField is present.
    expect(find.byType(TextField), findsOneWidget);

    // Enter some text into the TextField.
    await tester.enterText(find.byType(TextField), 'Hello, World!');
    await tester.pump();

    // Verify that the text was entered.
    expect(find.text('Hello, World!'), findsOneWidget);
  });
}
